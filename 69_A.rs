use std::io;

fn read_lines() -> Vec<i32> {
  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  line.trim().split(' ').flat_map(str::parse::<i32>).collect()
}

fn main(){
  let n = read_lines()[0];
  let mut x = 0;
  let mut y = 0;
  let mut z = 0;
  
  for _i in 0..n{
    let line = read_lines();
    x += line[0];
    y += line[1];
    z += line[2];
  }

  if (x==0) && (y==0) && (z==0){
    println!("YES");
  }
  else{
    println!("NO");
  }
}