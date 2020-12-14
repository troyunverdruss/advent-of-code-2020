use crate::util::inputs::day_input;
// use z3::{Config, Context, ast, SatResult};
// use z3::ast::{Int, Ast};

// TODO chinese remainder theorem? yes but how
pub fn run() {
    let lines = day_input(13);
    let min_bus = part1(&lines);

    println!(
        "Part 1: bus: {}, wait time: {}, mul: {}",
        min_bus.0,
        min_bus.1,
        min_bus.0 * min_bus.1
    );

    part2(&lines);
}

fn part1(lines: &[String]) -> (usize, usize) {
    let departure_time = lines.get(0).unwrap().parse::<usize>().unwrap();
    let buses = lines
        .get(1)
        .unwrap()
        .split(',')
        .filter(|v| v != &"x")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let min_bus = buses
        .iter()
        .map(|bus| {
            let remainder = departure_time % bus;
            (bus, bus - remainder)
        })
        .min_by(|a, b| a.to_owned().1.cmp(&b.to_owned().1))
        .unwrap();

    (min_bus.0.to_owned(), min_bus.1)
}

fn part2(lines: &[String]) -> i64 {
    let buses_with_index = lines
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|t| t.1 != "x")
        .map(|t| (t.0, t.1.parse::<usize>().unwrap()))
        .collect::<Vec<(usize, usize)>>();

    println!("{:?}", buses_with_index);

    let remainders: Vec<usize> = buses_with_index.iter().map(|t| t.0).collect();
    let divisors: Vec<usize> = buses_with_index.iter().map(|t| t.1).collect();

    println!("{:?}", divisors);
    println!("{:?}", remainders);
    // 56422801821895 too low
    // correct! 840493039281088
    // 896915841102983 too high

    // used chinese remainder theorem solver in python :/
    // from sympy.ntheory.modular import crt
    // crt([41, 37, 379, 23, 13, 17, 29, 557, 19], [0, 35, 41, 49, 54, 58, 70, 72, 91])

    0
}

#[cfg(test)]
mod tests {
    use crate::day13::{part1, part2};
    // use z3::{Config, Context, ast, SatResult, Sort, Symbol};
    // use z3::ast::{Ast, Real, Bool, Int};

    #[test]
    fn test_part1() {
        let lines = vec!["939".to_owned(), "7,13,x,x,59,x,31,19".to_owned()];

        let result = part1(&lines);
        assert_eq!(295, result.0 * result.1);
    }

    #[test]
    fn test_part2_1() {
        let lines = vec!["".to_owned(), "7,13,x,x,59,x,31,19".to_owned()];

        let result = part2(&lines);
        assert_eq!(1068781, result);
    }

    #[test]
    fn test_part2_2() {
        let lines = vec!["".to_owned(), "17,x,13,19".to_owned()];

        let result = part2(&lines);
        assert_eq!(3417, result);
    }

    // #[test]
    // fn test_z3() {
    //     let cfg = Config::new();
    //     let ctx = Context::new(&cfg);
    //
    //     let x = ctx.
    //
    //     let solver = z3::Solver::new(&ctx);
    //     solver.assert(&Bool::try_from());
    //
    //     let y = Real::new_const(&ctx, "f");
    //
    //
    //
    //
    //     let x = ast::Int::new_const(&ctx, "x");
    //     let mut y = ast::Int::from_i64(&ctx, 13);
    //     let xmody = x.modulo(&y);
    //     let zero: Int = ast::Int::from_i64(&ctx, 0);
    //     let www = y.sub_assign(&xmody);
    //     y.
    //     y.unary_minus(xmody);
    //
    //     let f = z3::FuncDecl::new(&ctx, "f", &[&Sort::int(&ctx)], &Sort::int(&ctx));
    //
    //     f.apply(&[1])
    //
    //     let bv = ast::BV::new_const(&ctx, "x", 32);
    //     solver.assert(&bv._eq(&ast::BV::from_i64(&ctx, -3, 32)));
    //
    //     let x = ast::Int::from_bv(&bv, true);
    //
    //     assert_eq!(solver.check(), SatResult::Sat);
    //     let model = solver.get_model().unwrap();
    //
    //     assert_eq!(-3, model.eval(&x).unwrap().as_i64().unwrap());
    // }
}
