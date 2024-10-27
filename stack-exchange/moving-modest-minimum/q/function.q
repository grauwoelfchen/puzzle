// BQN
// miccibaum.github.io/BQN

// converge <-> diverge
// fold/reduce

func: {[list]
  // min
  // (&/) list

  s: iasc list;
  m: s[0];
  n: s[1];

  // asc list;
  // {(asc x) x = min x} list

  // {s @ x = first s: asc x} list

  // {[a] {min x _ y}[a] each (til count a)} list

  // k
  // {[a] (&/ _)[a;]'!#a}

  // {[a] min (_ each)[a;] (til count a)} list
  // {[a] min_[a] each (til count a)} list

  // map?
  // list @ s
  // {[x] $[x >= n; m; n]}/ s

  //
  // ... experiments :'(

  // infinity loop?
  // ({[x] x + 1}/) list

  // {[x] $[x >= n; m; n]}/[{[x] x >= 0}; list]
  // {[x] x + 1}/[{[x] x >= 0}; list]
  // {[x] x + 1}/list

  // scan with if-else
  // ({[x; y] $[y >= n; m; n]}\) list

  // each with previous (0)
  // 0 {[this; pre] $[this >= n; m; n]}': list

  // another scan \
  // {[x] $[x >= 0; 0; 1]}\[{[x] x >= 0}; list]

  // 1 {[x] $[x >= 0; m; n]}\ list
  };

// res: func[4 3 2 5];
// show res;

show func[4 3 2 5];
