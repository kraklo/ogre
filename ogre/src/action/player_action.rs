use uuid::Uuid;

#[derive(Debug)]
pub enum PlayerAction {
    KeepHand,
    MulliganHand,
    MulliganChooseBottom(Vec<Uuid>),
}
