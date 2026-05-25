#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    PreGame(PreGamePhase),
    Untap,
    Upkeep,
    Draw,
    FirstMain,
    Combat,
    SecondMain,
    EndStep,
    Cleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreGamePhase {
    KeepOrMulligan,
    MulliganChooseBottom,
}
