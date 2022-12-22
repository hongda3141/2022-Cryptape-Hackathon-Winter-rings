use trie::{PatriciaTrie, DB as trieDB};
use hasher::HasherKeccak;

pub struct MPTTrie<DB: trieDB>(PatriciaTrie<DB, HasherKeccak>);

