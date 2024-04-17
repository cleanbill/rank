pub mod html_graph {
    use core::f64;
    use std::env;
    use std::fs;
    use std::io::Write;

    use crate::Number_Set;

    fn setup_args() -> String {
        let args: Vec<String> = env::args().collect();

        let qty_args_ok = args.len() == 2;

        if !qty_args_ok {
            println!("Usage: {} <list of numbers>", args[0]);
            println!("Using default values");
        } else {
            print!("Using {} ", args[1]);
        }

        if qty_args_ok {
            return args[1].clone();
        }

        String::from(
            "25,29,48,58,6,0,0,0,11,18,41,21,17,37,14,16,19,23,48,19,33,46,24,23,35,21,23,26,59,43,66,43,41,108,57,50,39,30,52,37,30,63,54,63,25,29,26,47,29,23,40,86,40,34,35,52,44,37,33,30,39,23,28,28,60"
        )
    }

    fn write_header(mut file: &fs::File, output_filename: &str) {
        let write_attempt = writeln!(
            file,
            r#"
        <html style="padding: 200px; color: brown; font-family: 'Franklin Gothic Medium';">
        <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
        <main>
            <div>
            <h5 id="med"></h5>
            <canvas id="myChart"></canvas>
            </div>
        </main>
        
        <script>
        
            const ctx = document.getElementById('myChart');
            const NUMBER_CFG = ["#
        );
        if write_attempt.is_err() {
            panic!("cannot write {} header", output_filename);
        }
    }

    fn write_data(
        mut file: &fs::File,
        output_filename: &str,
        numbers_sets: Vec<Number_Set>,
        //string_array: String,
        labels: Vec<String>
    ) {
        // let string_array = setup_args();

        // let numbers: Vec<f64> = string_array
        //     .split(',')
        //     .map(|s| s.parse::<f64>().expect("Cannot parse numbers"))
        //     .collect();

        for number_set in numbers_sets {
            let numbers = number_set.numbers;
            for x in numbers.iter() {
                let write_attempt = writeln!(file, "{},", x);
                if write_attempt.is_err() {
                    panic!("cannot write {} data {}", output_filename, x);
                }
            }
        }
        let const_write = writeln!(file, "];\n\nconst labels = [");
        if const_write.is_err() {
            panic!("cannot write {} const ", output_filename);
        }

        let mut index = 1;
        // for _x in numbers.iter() {
        for label in labels {
            let write_attempt = write!(file, "\"{}\",", label);
            if write_attempt.is_err() {
                panic!("cannot write {} index {}", output_filename, index);
            }
            index = index + 1;
        }
    }

    fn write_footer(mut file: &fs::File, output_filename: &str) {
        let footer_write = writeln!(
            file,
            r#"];new Chart(ctx, {{
    type: 'line',
    data: {{
        labels: labels,
        datasets: [

            {{
                label: 'Rank',
                data: NUMBER_CFG,
            }},
            {{
                label: 'Bank',
                data: NUMBER_CFG,
            }},

        ]
    }}
}});
const median = arr => {{
    const mid = Math.floor(arr.length / 2),
        nums = [...arr].sort((a, b) => a - b);
    return arr.length % 2 !== 0 ? nums[mid] : (nums[mid - 1] + nums[mid]) / 2;
}};
const medElement = document.getElementById('med');
medElement.innerHTML = 'Median is ' + median(NUMBER_CFG);
</script>
</html>"#
        );
        if footer_write.is_err() {
            panic!("cannot write {} footer", output_filename);
        }
    }

    pub fn generate(output_filename: &str, numbers: Vec<Number_Set>, labels: Vec<String>) {
        //let output_filename = "line_graph.html";

        let file = fs::File::create(output_filename).unwrap();

        write_header(&file, &output_filename);
        write_data(&file, &output_filename, numbers, labels);
        write_footer(&file, &output_filename);
    }
}
