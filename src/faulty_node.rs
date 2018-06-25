use std::fmt::{self, Debug, Formatter};

/// Describes the reason why a Node is considered faulty.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaultKind {
    /// A given decryption share does not match its sender's public key.
    IncorrectDecryptionShareSender,
    /// Unable to deserailize the ciphertext received from a proposer.
    InvalidCiphertext,
    /// Unable to decrypt a share received from a proposer.
    ShareDecryptionFailed,
}

/// A collection of data representing the context of a faulty Node. This
/// structure describes which Node is faulty (`node_id`), why it is faulty
/// ('reason'), and when the fault occured (`epoch`).
pub struct FaultyNode<NodeUid> {
    pub node_id: NodeUid,
    pub reason: FaultKind,
    pub epoch: u64,
}

impl<NodeUid> Debug for FaultyNode<NodeUid> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.reason)
    }
}

/// A structure used to contain all occurences of faulty Node behavior.
#[derive(Debug)]
pub struct FaultyNodeLog<NodeUid>(
    pub Vec<FaultyNode<NodeUid>>
);

impl<NodeUid: Clone + Eq> FaultyNodeLog<NodeUid> {
    /// Creates an empty faulty Node log.
    pub fn new() -> Self {
        FaultyNodeLog(vec![])
    }

    /// Appends a new faulty Node occurence to the faulty Node log.
    pub fn append(&mut self, node_id: &NodeUid, reason: FaultKind, epoch: u64) {
        let faulty_node = FaultyNode { node_id: node_id.clone(), reason, epoch };
        self.0.push(faulty_node);
    }

    /// Returns all `node_id`s in the faulty Node log.
    pub fn node_ids(&self) -> Vec<NodeUid> {
        self.0.iter().map(|log| log.node_id.clone()).collect()
    }

    /// Returns whether or not `node_id` is a faulty Node.
    pub fn contains_node(&self, node_id: &NodeUid) -> bool {
        self.node_ids().contains(node_id)
    }
}
