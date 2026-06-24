# VibeTrust: The Anti-Flake Network

## Project Title
VibeTrust: The Anti-Flake Network

## Project Description
VibeTrust is a decentralized social accountability and immutable "receipts" network built on Soroban and the Stellar blockchain. It allows peer groups and project teams to log commitments, track broken promises (flakes), and collectively vote on social accountability metrics. All records and voting consensus are anchored on-chain, ensuring complete transparency, auditability, and resistance to manipulation or denial.

The platform combines a casual, high-vibe social interface with blockchain-backed reputation mechanics, creating a trusted ecosystem for group work, social scheduling, and everyday commitments.

## Project Vision
The vision of VibeTrust is to build a transparent, community-governed ledger for everyday promises and commitments. By leveraging blockchain technology, the platform eliminates the "he-said-she-said" dilemma in social or academic settings, preventing individuals from backtracking on agreements, dodging group work assignments, or ghosting plans.

VibeTrust aims to become the definitive "Hall of Shame & Fame" archive where communities can foster real accountability, celebrate reliable contributors, and maintain an unalterable history of peer trust on-chain.

## Key Features

### On-Chain Incident Logging
Users can log specific instances of unreliability or broken promises (e.g., missing group assignment deadlines, ghosting plans) directly onto the ledger.

### One Wallet, One Vote
To prevent spam, witch-hunting, or review manipulation, each unique Stellar wallet can submit only one active vote per incident to validate or dismiss the report.

### Decentralized Consensus Lock
Incidents remain in a pending state until a community consensus threshold is reached. Once verified, the case is permanently locked into the immutable "Hall of Shame."

### Social Reputation Profile
Users maintain an on-chain identity where their reliability is publicly auditable. Consistent positive behavior builds "Trust Score," while locked infractions permanently decay reputation.

### Cryptographic Accountability
Anonymous trolling is eliminated. Every incident report, defense, and vote requires an authentic cryptographic signature via a Web3 Stellar wallet.

### Public Trust Statistics
Anyone can query public records to view:
* Total incidents reported vs. finalized.
* Individual community trust rankings.
* Group project contribution verifications.

---

## Usage Instructions

### 1. Deploy Contract
Deploy the Soroban smart contract and initialize the system environment.

### 2. File a Report (Create Incident)
An aggrieved user connects their Stellar wallet and logs a specific incident against an offender’s wallet address, accompanied by a context string.

### 3. Cast Community Votes
Peer group members connect their wallets to review the pending incident and vote `True` (Guilty) or `False` (Innocent).

### 4. Finalize the Ledger (Lock Case)
Once an incident accumulates enough penalty votes ($\ge 3$) and holds a clear majority, any user can execute the lock function to permanently archive the record.

### 5. Query Trust Profiles
Input any wallet address to fetch its real-time social credit status, active incidents, and locked history.

---

## Future Scope

### Smart Escrow Contracts (Soroban Escrow)
Allow users to lock a nominal amount of XLM or custom `VIBE` tokens into a contract when making a commitment. If they flake, the stake is automatically distributed to the affected peers.

### Soulbound Achievement Badges (SBTs)
Issue non-transferable NFT badges to profiles, such as "Ultimate Flake" for repeat offenders or "Rock Solid" for perfect project contributors.

### Proof of Contribution for Group Projects
Integrate directly into project management tools (like GitHub or Trello) to log task completion statuses straight to Stellar for academic/professional grading audits.

### Advanced Weighted Reputation
Weight community votes based on the voter’s own on-chain trust score to prevent coordinated bot attacks or collusive voting.

### Curated Peer Circles
Allow users to establish private multisig-backed "Circles" (e.g., University Classrooms, Friend Groups) where voting rules and thresholds are customized.

---

## Technology Stack
* **Rust** for robust smart contract engineering.
* **Soroban SDK** for specialized, low-cost blockchain logic execution.
* **Stellar Blockchain** for ultra-fast, decentralized, secure state storage.
* **Stellar Wallets (e.g., Freighter)** for secure cryptographic identity and authentication.
* **Web Frontend** (React, Tailwind CSS) for smooth, gamified community interactions.

## Contribution
Community contributions are highly encouraged from blockchain engineers, UI/UX designers, and open-source advocates. Fork the repository and submit pull requests to help optimize VibeTrust and foster absolute accountability through decentralized tech.

## License
This project is licensed under the MIT License.

## Contract Detail

### Incident Registration
Handles the creation of unique incident identifiers, linking victims, offenders, and context payloads.

### Voting Logic
Manages peer verification inputs while strictly enforcing anti-double-voting measures per address.

### State Consolidation (Locking Mechanism)
Calculates consensus parameters and permanently freezes authenticated receipts within Instance Storage.

### Profile Queries
Exposes optimized public endpoints to feed real-time contract states, tallies, and configurations straight to the Frontend interface.

**Contract ID:** CBNPIAC2LI2R5NZGPA7VNKEFKVTXBUOFDLROKYS3523KXHJ5DAU7OGZF
### Contact me

**Facebook:** https://www.facebook.com/eirakimura
**Gmail:** vnender2007@gmail.com
**Number:** 0375300018
