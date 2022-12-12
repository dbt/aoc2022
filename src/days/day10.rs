use adventools::prelude::*;

struct Machine {
    instructions: Vec<String>,
    ip: usize,
    cycle: usize,
    ctr: usize,
    reg_x: i32,
}

impl Machine {
    fn new(inst: Vec<String>) -> Self {
        Machine {
            instructions: inst,
            ip: 0,
            cycle: 0,
            ctr: 0,
            reg_x: 1,
        }
    }

    fn tick(&mut self) -> i32 {
        let cur = self.reg_x;
        self.ctr += 1;
        self.cycle += 1;
        let mut op = self.instructions[self.ip].split(" ");
        let inst = op.next().unwrap();

        let completed = match inst {
            "noop" => true,
            "addx" => {
                if self.ctr == 2 {
                    let d: i32 = op.next().unwrap().parse().unwrap();
                    self.reg_x += d;
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        };
        if completed {
            self.ctr = 0;
            self.ip += 1;
        }
        cur
    }

    fn signal_during(&mut self, cycle_dest: usize) -> i32 {
        let mut out = 0;
        while self.cycle < cycle_dest {
            out = self.tick();
        }
        out * cycle_dest as i32
    }

    fn signal_readings(&mut self, cycles: &[usize]) -> i32 {
        let val: i32 = cycles.iter().map(|&v| self.signal_during(v)).sum();
        val
    }

    fn draw(&mut self, w: usize, h: usize) -> Vec<String> {
        let mut v = vec![];
        for _y in 0..h {
            let mut s = String::new();
            for x in 0..(w as i32) {
                let val = self.tick();
                if x + 1 < val || x - 1 > val {
                    s.push('.');
                } else {
                    s.push('#');
                }
            }
            v.push(s);
        }
        v
    }
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        10
    }
    fn part01(&self) -> Result<()> {
        let prog = read_lines("input10.txt")?;
        let mut m = Machine::new(prog);
        println!("{}", m.signal_readings(&vec![20, 60, 100, 140, 180, 220]));
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let prog = read_lines("input10.txt")?;
        let mut m = Machine::new(prog);
        println!(
            "\n{}",
            m.draw(40, 6).join("\n").replace("#", "█").replace(".", " ")
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    fn test_data() -> Vec<String> {
        read_lines("test10.txt").unwrap()
    }

    #[test]
    fn test_machine() {
        let inst = split_str(
            r"noop
addx 3
addx -5",
        );
        let mut m = Machine::new(inst);
        m.tick();
        assert_eq!(1, m.reg_x);
        assert_eq!(1, m.cycle);
        assert_eq!(1, m.ip);
        m.tick();
        m.tick();
        assert_eq!(4, m.reg_x);
        assert_eq!(3, m.cycle);
        assert_eq!(2, m.ip);
        m.tick();
        m.tick();
        assert_eq!(-1, m.reg_x);
        assert_eq!(5, m.cycle);
        assert_eq!(3, m.ip);
    }

    #[test]
    fn test_p1() {
        let mut m = Machine::new(test_data());
        assert_eq!(m.signal_readings(&vec![20, 60, 100, 140, 180, 220]), 13140);
    }
    #[test]
    fn test_p2() {
        let mut m = Machine::new(test_data());
        let expected = split_str(
            r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
",
        );
        let val = m.draw(40, 6);
        let diff: Vec<String> = zip(val, expected)
            .map(|(s1, s2)| {
                zip(s1.chars(), s2.chars())
                    .map(|(c1, c2)| {
                        if c1 == c2 {
                            c1
                        } else if c1 < c2 {
                            '<'
                        } else {
                            '>'
                        }
                    })
                    .collect()
            })
            .collect();
        let diffs = diff.join("\n");
        println!("{}", diff.join("\n"));
        assert!(!diffs.contains('<'));
        assert!(!diffs.contains('>'));
        //        assert_eq!(val, expected);
    }
}
