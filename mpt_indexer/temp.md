做一个所有的 live cell 的 MPT indexer
 
key 是 blake2b(out_point | encode(cell)) val 是 bool 
这样我可以根据 out point 和 cell 放在一块生成 proof
只要 Axon 上面有 mpt root 就可以验证 proof，进而验证 cell 是 live cell
  
很简单，参考 ckb-indexer 主要处理好回滚逻辑
 
怎么提供服务呢
还有个信任问题
貌似信任问题不是很大
  
信任问题标准比较清晰,就是提供服务的方式,
  
正在考虑,那提供rpc会不会有中心化的问题 
 
我觉得做成库更好一点啊
rpc_client 和 store 都抽象成 trait