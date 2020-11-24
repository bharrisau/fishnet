use arrayvec::ArrayString;
use std::fmt;
use std::time::Duration;
use std::str::FromStr;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::variants::Variant;
use tokio::sync::oneshot;
use crate::api::Score;
use crate::stockfish::StockfishError;

/// Uniquely identifies a batch in this process.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BatchId(ArrayString<[u8; 16]>);

impl FromStr for BatchId {
    type Err = arrayvec::CapacityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatchId(s.parse()?))
    }
}

impl fmt::Display for BatchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Uniquely identifies a position within a batch.
#[derive(Debug, Clone)]
pub struct PositionId(pub usize);

#[derive(Debug, Clone)]
pub struct Skill(u32);

#[derive(Debug, Clone)]
pub struct Position {
    pub batch_id: BatchId,
    pub position_id: PositionId,

    pub variant: Variant,
    pub fen: Option<Fen>,
    pub moves: Vec<Uci>,
    pub nodes: u64,
    pub skill: Option<Skill>,
}

#[derive(Debug, Clone)]
pub struct PositionResponse {
    pub batch_id: BatchId,
    pub position_id: PositionId,

    pub score: Score,
    pub best_move: Option<Uci>,
    pub pv: Vec<Uci>,
    pub depth: u32,
    pub nodes: u64,
    pub time: Duration,
    pub nps: Option<u32>,
}

#[derive(Debug)]
pub struct PositionFailed {
    batch_id: BatchId,
}

#[derive(Debug)]
pub struct Pull {
    pub response: Option<Result<PositionResponse, PositionFailed>>,
    pub callback: oneshot::Sender<Position>,
}
