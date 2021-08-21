use mongo_model::PotentialCandidate;

struct PartyCandidates {
    party: String,
    candidates: Vec<PotentialCandidate>,
}