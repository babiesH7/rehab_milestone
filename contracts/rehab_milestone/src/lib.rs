#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

/// Information about a rehabilitation program, jointly governed by a
/// therapist and a patient.
#[contracttype]
#[derive(Clone)]
pub struct ProgramInfo {
    pub therapist: Address,
    pub patient: Address,
    pub active: bool,
}

/// Status of a single recovery milestone within a program.
/// A milestone is `claimed` by the patient first, then `confirmed`
/// by the therapist. Once confirmed, the symbolic reward can be claimed.
#[contracttype]
#[derive(Clone)]
pub struct MilestoneStatus {
    pub claimed: bool,
    pub confirmed: bool,
    pub reward_claimed: bool,
}

/// Storage keys used by the contract. Using an enum keeps the keyspace
/// tidy and avoids accidental collisions between different data domains.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Program metadata keyed by program_id.
    Program(Symbol),
    /// Milestone status keyed by (program_id, milestone_id).
    Milestone(Symbol, Symbol),
    /// Running count of confirmed milestones keyed by program_id.
    Count(Symbol),
}

#[contract]
pub struct RehabMilestone;

#[contractimpl]
impl RehabMilestone {
    /// Open a new rehabilitation program that binds a therapist and a
    /// patient together under a unique `program_id`. The therapist must
    /// sign the transaction. The same `program_id` cannot be reused.
    pub fn create_program(
        env: Env,
        therapist: Address,
        patient: Address,
        program_id: Symbol,
    ) {
        therapist.require_auth();

        let prog_key = DataKey::Program(program_id.clone());
        if env.storage().persistent().has(&prog_key) {
            panic!("program already exists");
        }

        let info = ProgramInfo {
            therapist,
            patient,
            active: true,
        };
        env.storage().persistent().set(&prog_key, &info);
        env.storage()
            .persistent()
            .set(&DataKey::Count(program_id), &0u32);
    }

    /// The patient declares that they have achieved a milestone, for
    /// example "walk 100m unaided" or "lift 5kg with injured arm".
    /// The achievement is recorded but not yet counted toward progress
    /// until the therapist confirms it.
    pub fn achieve_milestone(
        env: Env,
        patient: Address,
        program_id: Symbol,
        milestone_id: Symbol,
    ) {
        patient.require_auth();

        let info: ProgramInfo = env
            .storage()
            .persistent()
            .get(&DataKey::Program(program_id.clone()))
            .unwrap_or_else(|| panic!("program not found"));

        if !info.active {
            panic!("program is closed");
        }
        if info.patient != patient {
            panic!("only the enrolled patient can claim a milestone");
        }

        let m_key = DataKey::Milestone(program_id, milestone_id);
        let mut status: MilestoneStatus =
            env.storage().persistent().get(&m_key).unwrap_or(MilestoneStatus {
                claimed: false,
                confirmed: false,
                reward_claimed: false,
            });

        if status.confirmed {
            panic!("milestone already confirmed");
        }

        status.claimed = true;
        env.storage().persistent().set(&m_key, &status);
    }

    /// The therapist co-signs the milestone the patient previously
    /// claimed. Confirmation is what officially unlocks reward
    /// eligibility and increments the program's confirmed milestone
    /// counter.
    pub fn confirm_milestone(
        env: Env,
        therapist: Address,
        patient: Address,
        program_id: Symbol,
        milestone_id: Symbol,
    ) {
        therapist.require_auth();

        let info: ProgramInfo = env
            .storage()
            .persistent()
            .get(&DataKey::Program(program_id.clone()))
            .unwrap_or_else(|| panic!("program not found"));

        if info.therapist != therapist {
            panic!("only the assigned therapist can confirm");
        }
        if info.patient != patient {
            panic!("patient does not match this program");
        }
        if !info.active {
            panic!("program is closed");
        }

        let m_key = DataKey::Milestone(program_id.clone(), milestone_id);
        let mut status: MilestoneStatus = env
            .storage()
            .persistent()
            .get(&m_key)
            .unwrap_or_else(|| panic!("patient has not claimed this milestone yet"));

        if !status.claimed {
            panic!("patient must claim before therapist confirms");
        }
        if status.confirmed {
            panic!("milestone already confirmed");
        }

        status.confirmed = true;
        env.storage().persistent().set(&m_key, &status);

        let count_key = DataKey::Count(program_id);
        let current: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
        env.storage().persistent().set(&count_key, &(current + 1));
    }

    /// The patient claims the symbolic reward badge tied to a confirmed
    /// milestone. No real XLM is transferred; this only flips an
    /// on-chain flag that downstream off-chain systems (insurance,
    /// employer wellness programs, charities) can verify as proof of
    /// recovery progress.
    pub fn claim_reward(
        env: Env,
        patient: Address,
        program_id: Symbol,
        milestone_id: Symbol,
    ) -> bool {
        patient.require_auth();

        let info: ProgramInfo = env
            .storage()
            .persistent()
            .get(&DataKey::Program(program_id.clone()))
            .unwrap_or_else(|| panic!("program not found"));
        if info.patient != patient {
            panic!("only the patient can claim a reward");
        }

        let m_key = DataKey::Milestone(program_id, milestone_id);
        let mut status: MilestoneStatus = env
            .storage()
            .persistent()
            .get(&m_key)
            .unwrap_or_else(|| panic!("milestone not found"));

        if !status.confirmed {
            panic!("milestone has not been confirmed by the therapist");
        }
        if status.reward_claimed {
            panic!("reward already claimed for this milestone");
        }

        status.reward_claimed = true;
        env.storage().persistent().set(&m_key, &status);
        true
    }

    /// Close an active rehab program. After closing, no further
    /// milestones can be claimed or confirmed. Only the therapist who
    /// originally opened the program may close it.
    pub fn close_program(env: Env, therapist: Address, program_id: Symbol) {
        therapist.require_auth();

        let prog_key = DataKey::Program(program_id);
        let mut info: ProgramInfo = env
            .storage()
            .persistent()
            .get(&prog_key)
            .unwrap_or_else(|| panic!("program not found"));

        if info.therapist != therapist {
            panic!("only the therapist may close the program");
        }

        info.active = false;
        env.storage().persistent().set(&prog_key, &info);
    }

    /// Returns the total number of milestones in the program that have
    /// been confirmed by the therapist.
    pub fn milestone_count(env: Env, program_id: Symbol) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Count(program_id))
            .unwrap_or(0)
    }

    /// Returns true if the given milestone has been jointly approved
    /// (patient claimed + therapist confirmed).
    pub fn is_confirmed(env: Env, program_id: Symbol, milestone_id: Symbol) -> bool {
        env.storage()
            .persistent()
            .get::<DataKey, MilestoneStatus>(&DataKey::Milestone(program_id, milestone_id))
            .map(|s| s.confirmed)
            .unwrap_or(false)
    }

    /// Returns true if the patient has already redeemed the reward
    /// flag for a milestone.
    pub fn is_reward_claimed(
        env: Env,
        program_id: Symbol,
        milestone_id: Symbol,
    ) -> bool {
        env.storage()
            .persistent()
            .get::<DataKey, MilestoneStatus>(&DataKey::Milestone(program_id, milestone_id))
            .map(|s| s.reward_claimed)
            .unwrap_or(false)
    }

    /// Returns the ProgramInfo for a given program_id, useful for UIs
    /// to display the bound therapist/patient pair and program status.
    pub fn get_program(env: Env, program_id: Symbol) -> ProgramInfo {
        env.storage()
            .persistent()
            .get(&DataKey::Program(program_id))
            .unwrap_or_else(|| panic!("program not found"))
    }
}
