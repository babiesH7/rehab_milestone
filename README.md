# rehab_milestone

## Project Title
rehab_milestone — On-Chain Rehabilitation Milestone Tracker

## Project Description
Patients recovering from injury, surgery, or addiction often need third parties
(insurers, employers, charities, family members) to trust that they are
genuinely progressing through a rehab program before unlocking benefits, time
off, or financial support. Today that trust is delivered through easily-forged
paper notes or siloed clinic portals. **rehab_milestone** is a Soroban smart
contract that records every recovery milestone as a two-signature event:
the patient self-reports the achievement, and the licensed therapist co-signs
to confirm it. The result is a tamper-proof, publicly verifiable record of
recovery progress on the Stellar network.

## Project Vision
We envision a healthcare ecosystem where recovery progress is a portable,
patient-owned credential rather than a proprietary record locked inside a
single clinic. By turning each confirmed milestone into a transparent on-chain
event, **rehab_milestone** lays the foundation for value-based care payments,
fairer return-to-work assessments, micro-insurance payouts that release on
real outcomes, and motivational reward programs that celebrate concrete
recovery wins instead of mere appointment attendance. In the long term we
aim to make verifiable, patient-controlled rehab data the default standard
across physiotherapy, post-surgical recovery, and behavioral health.

## Key Features
- **Joint patient + therapist sign-off** — milestones require both
  `achieve_milestone` (patient signs) and `confirm_milestone`
  (therapist signs) before they count, eliminating self-reporting fraud.
- **Persistent on-chain ledger** — each program and its milestones are stored
  in Soroban persistent storage, so the recovery history survives clinic
  changes and remains auditable forever.
- **Symbolic reward badges** — once a milestone is confirmed, the patient can
  call `claim_reward` to flip an on-chain flag that external programs
  (insurers, gyms, employers) can read as proof to release benefits — no
  real XLM is moved by the contract.
- **Program lifecycle control** — therapists can `close_program` when a
  course of treatment ends, freezing further changes while keeping history
  readable.
- **Public read endpoints** — `milestone_count`, `is_confirmed`,
  `is_reward_claimed`, and `get_program` let any frontend or third-party
  verifier audit a patient's verified progress without ever needing private
  clinical notes.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** healthcare dApp — see `contracts/rehab_milestone/src/lib.rs` for the full rehab_milestone business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CARRHZFWR4TQNO73AQTLE2IB5MUZML345Y3VSJ2QZSTLJVLNKQHVCQTW`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/a5813403f1be6ba344ebf4c1930b75b3ebbcb85fda322c4bb0e5bc3c88b21977`

## Future Scope
- **Real reward payouts** — integrate Stellar's native asset and trustlines so
  that confirmed milestones can automatically release USDC or a clinic-issued
  recovery token to the patient's wallet.
- **Multi-signer programs** — extend confirmation to require multiple
  clinicians (e.g. physiotherapist + physician) for high-value milestones.
- **NFT recovery certificates** — mint an SEP-39-style asset for each fully
  completed program as a portable, verifiable rehabilitation credential.
- **Privacy layer** — store only hashed milestone descriptions on-chain with
  encrypted off-chain detail (IPFS) to keep clinical specifics private while
  remaining auditable.
- **Insurance & employer integrations** — publish reference adapters so
  insurers can subscribe to `confirm_milestone` events and trigger
  policy-defined disbursements automatically.
- **Mobile patient frontend** — a Freighter-connected web app and React Native
  client so patients can claim milestones directly from a phone after each
  therapy session.
- **DAO governance for community rehab programs** — let charities and
  community health initiatives govern shared rehab program parameters
  (milestone definitions, reward sizes) through token-weighted voting.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `rehab_milestone` (healthcare)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
