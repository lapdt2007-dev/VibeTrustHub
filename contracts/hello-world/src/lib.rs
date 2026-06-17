#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, env, Address, String, Symbol, Vec};

// 1. Định nghĩa cấu trúc dữ liệu cho một vụ "Bóc Phốt" (Incident)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Incident {
    pub id: u32,
    pub victim: Address,      // Người bị leo cây / người tố cáo
    pub offender: Address,    // Kẻ lật lọng
    pub content: String,      // Nội dung thù vặt (Ví dụ: "Hứa bao gà rán xong trốn")
    pub upvotes: u32,         // Số phiếu đồng tình (Phạt)
    pub downvotes: u32,       // Số phiếu bênh vực (Oan uổng)
    pub is_locked: bool,      // Nếu true = Đã lên sổ cái vĩnh viễn, không ai cãi được nữa
}

// 2. Định nghĩa các Khóa (Keys) để lưu trữ dữ liệu vào Blockchain State
#[contracttype]
pub enum DataKey {
    IncidentCount,            // Biến đếm tổng số vụ để tự động tăng ID
    Incident(u32),            // Lưu chi tiết từng vụ theo ID
    HasVoted(u32, Address),   // Đánh dấu xem một người đã Vote cho vụ ID này chưa (Tránh spam vote)
}

#[contract]
pub struct AntiLuonContract;

#[contractimpl]
impl AntiLuonContract {

    /// **Tính năng 1: Tạo một vụ bóc phốt mới**
    /// Người tố cáo (victim) sẽ ghi lại hành vi của kẻ lật lọng (offender)
    pub fn create_incident(env: Env, victim: Address, offender: Address, content: String) -> u32 {
        // Xác thực chữ ký của người tố cáo để đảm bảo không ai mạo danh họ
        victim.require_auth();

        // Lấy ID tiếp theo (Tự động tăng)
        let mut count: u32 = env.storage().instance().get(&DataKey::IncidentCount).unwrap_or(0);
        count += 1;

        let new_incident = Incident {
            id: count,
            victim: victim.clone(),
            offender: offender.clone(),
            content,
            upvotes: 0,
            downvotes: 0,
            is_locked: false,
        };

        // Lưu vào bộ nhớ của Smart Contract
        env.storage().instance().set(&DataKey::Incident(count), &new_incident);
        env.storage().instance().set(&DataKey::IncidentCount, &count);

        // Phát ra một Event để Frontend (Giao diện) có thể bắt được và hiển thị realtime
        env.events().publish((Symbol::new(&env, "new_phot"), count), (victim, offender));

        count // Trả về ID của vụ phốt vừa tạo
    }

    /// **Tính năng 2: Hội bạn thân vào Vote**
    /// `vote_type`: true là "Nó lươn thật, phạt nó!", false là "Nó oan, tha cho nó!"
    pub fn vote_incident(env: Env, voter: Address, incident_id: u32, vote_type: bool) {
        voter.require_auth();

        // Kiểm tra xem vụ phốt này có tồn tại không
        let mut incident: Incident = env
            .storage()
            .instance()
            .get(&DataKey::Incident(incident_id))
            .unwrap_or_else(|| panic!("Vụ phốt này không tồn tại!"));

        // Kiểm tra xem vụ này đã bị khóa (chốt án) chưa
        if incident.is_locked {
            panic!("Vụ này đã chốt án và ghi vào sổ đoạn trường, không được vote nữa!");
        }

        // Kiểm tra xem người này đã vote cho vụ này chưa
        let vote_key = DataKey::HasVoted(incident_id, voter.clone());
        if env.storage().instance().has(&vote_key) {
            panic!("Bạn đã vote cho vụ này rồi, bớt spam!");
        }

        // Cập nhật số phiếu
        if vote_type {
            incident.upvotes += 1;
        } else {
            incident.downvotes += 1;
        }

        // Đánh dấu là người này đã vote
        env.storage().instance().set(&vote_key, &true);

        // Cập nhật lại trạng thái vụ phốt
        env.storage().instance().set(&DataKey::Incident(incident_id), &incident);

        env.events().publish((Symbol::new(&env, "voted"), incident_id), voter);
    }

    /// **Tính năng 3: Chốt án (Lock Incident)**
    /// Khi số phiếu phạt (upvotes) đạt ngưỡng (Ví dụ: từ 3 phiếu trở lên và vượt trội so với downvotes),
    /// bất kỳ ai cũng có thể gọi hàm này để khóa cứng vụ phốt vào "Đại lộ Ngang trái" vĩnh viễn.
    pub fn lock_incident(env: Env, incident_id: u32) {
        let mut incident: Incident = env
            .storage()
            .instance()
            .get(&DataKey::Incident(incident_id))
            .unwrap_or_else(|| panic!("Vụ phốt không tồn tại"));

        if incident.is_locked {
            panic!("Đã khóa rồi, không cần khóa lại đâu.");
        }

        // Logic check: Số phiếu phạt phải >= 3 và lớn hơn số phiếu bênh vực
        if incident.upvotes >= 3 && incident.upvotes > incident.downvotes {
            incident.is_locked = true;
            env.storage().instance().set(&DataKey::Incident(incident_id), &incident);
            
            // Bắn event: Kẻ lật lọng chính thức bị đóng đinh trên Blockchain!
            env.events().publish(
                (Symbol::new(&env, "shamed_forever"), incident.offender.clone()),
                incident_id
            );
        } else {
            panic!("Chưa đủ bằng chứng và sự đồng thuận từ hội bạn để chốt tội!");
        }
    }

    /// **Tính năng phụ: Đọc dữ liệu để hiển thị lên UI**
    pub fn get_incident(env: Env, incident_id: u32) -> Incident {
        env.storage()
            .instance()
            .get(&DataKey::Incident(incident_id))
            .unwrap_or_else(|| panic!("Không tìm thấy dữ liệu!"))
    }
}