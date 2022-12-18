macro_rules! make_runner {
    (@helper
        { $($mods:tt)* }
        { $($labels:tt)* }
        { $($arms:tt)* }
        $day:tt,
        $($rest:tt)*
    ) => (
        paste::paste! { crate::utils::make_runner!(@helper
            {
                $($mods)*
                mod [<day $day>];
            }
            {
                [< Day $day >],
                $($labels)*
            }
            {
                Task::[< Day $day >] => {
                    let input = include_str!(concat!("../inputs/", $day, ".txt"));
                    (stringify!($day), [< day $day >]::solve(input).to_string())
                },
                $($arms)*
            }
            $($rest)*
        ); }
    );
    (@helper
        { $($mods:tt)* }
        { $($labels:tt)* }
        { $($arms:tt)* }
        $day:tt +,
        $($rest:tt)*
    ) => (
        paste::paste! { crate::utils::make_runner!(@helper
            {
                $($mods)*
                mod [<day $day>];
            }
            {
                [< Day $day _2 >],
                [< Day $day >],
                $($labels)*
            }
            {
                Task::[< Day $day _2 >] => {
                    let input = include_str!(concat!("../inputs/", $day, ".txt"));
                    (concat!($day, " (part 2)"), [< day $day >]::solve_2(input).to_string())
                },
                Task::[< Day $day >] => {
                    let input = include_str!(concat!("../inputs/", $day, ".txt"));
                    (stringify!($day), [< day $day >]::solve(input).to_string())
                },
                $($arms)*
            }
            $($rest)*
        ); }
    );
    (@helper
        { $($mods:tt)* }
        { $($labels:tt)* }
        { $lhs:path => $rhs:expr, $($rest_lhs:path => $rest_rhs:expr,)* }
    ) => (
        #[derive(clap::ValueEnum, Copy, Clone, Debug)]
        enum Task { $($labels)* Latest }

        $($mods)*

        fn run(args: Args) {
            let start = std::time::Instant::now();
            let (day, result) = match args.task {
                $lhs => $rhs,
                Task::Latest => $rhs,
                $($rest_lhs => $rest_rhs,)*
            };
            let duration = start.elapsed().as_secs_f32();
            println!("Computed result for day {day} in {duration:.3} seconds: {result}");
        }
    );

    ($($day:tt)*) => {
        crate::utils::make_runner!(@helper {} {} {} $($day)*);
    };
}

pub(crate) use make_runner;
