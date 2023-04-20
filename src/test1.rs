use std::fs;
use crate::simple::SimpleGraph;
use crate::graph::Graph;
use crate::coarse::CoarseCSRGraph;
use crate::graph::GraphErr;


#[test]
fn test() {let val = g.add_node(0);
assert!(val.is_err());
let val = g.add_node(1);
assert!(val.is_err());
let val = g.add_node(2);
assert!(val.is_err());
let val = g.add_edge(2,1,0.3012501110619543);
assert!(val.is_some())
let val = g.add_edge(2,1,0.9327723015617692);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,2,0.7342436343441496);
assert!(val.is_some())
let val = g.add_edge(1,2,0.6724177981351609);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_node(3);
assert!(val.is_err());
let val = g.add_edge(1,0,0.38658113464759014);
assert!(val.is_some())
let val = g.add_edge(1,0,0.6816604621503433);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(2,0,0.7652064727874887);
assert!(val.is_some())
let val = g.add_edge(2,0,0.18788355870927487);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,3,0.2505402959711931);
assert!(val.is_some())
let val = g.add_edge(1,3,0.38081290645931953);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_node(4);
assert!(val.is_err());
let val = g.add_edge(1,2,0.9291878986621727);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,2,0.36956409568130266);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_node(5);
assert!(val.is_err());
let val = g.add_edge(1,2,0.41684603739203896);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,2,0.6345317120425951);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(3,1,0.822712225413352);
assert!(val.is_some())
let val = g.add_edge(3,1,0.2525560942756363);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(2,4,0.3740136573744445);
assert!(val.is_some())
let val = g.add_edge(2,4,0.3791321732212921);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,0,0.6818113826751719);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,0,0.9340752610857586);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_node(6);
assert!(val.is_err());
let val = g.add_edge(2,5,0.9436819751018618);
assert!(val.is_some())
let val = g.add_edge(2,5,0.13815145389104122);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(5,0,0.14111979708657685);
assert!(val.is_some())
let val = g.add_edge(5,0,0.08307221917768359);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_node(7);
assert!(val.is_err());
let val = g.add_edge(2,1,0.6149190321761803);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(2,1,0.5093947649049306);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(3,2,0.5301690100198916);
assert!(val.is_some())
let val = g.add_edge(3,2,0.8179118461942007);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(2,4,0.8796735788437504);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(2,4,0.4632582631170543);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_node(8);
assert!(val.is_err());
let val = g.add_node(9);
assert!(val.is_err());
let val = g.add_edge(7,5,0.6905383560556521);
assert!(val.is_some())
let val = g.add_edge(7,5,0.0073003361503323205);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,2,0.253806744524231);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,2,0.8073129307721926);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(8,8,0.8595745994291696);
assert!(val.is_some())
let val = g.add_edge(8,8,0.4983823206740263);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(1,5,0.6022355446470424);
assert!(val.is_some())
let val = g.add_edge(1,5,0.0749536871354406);
assert!(val == GraphErr::EdgeAlreadyExists)
let val = g.add_edge(3,5,0.30647191864177525);
assert!(val.is_some())
let val = g.add_edge(3,5,0.23056005576921978);
assert!(val == GraphErr::EdgeAlreadyExists)

