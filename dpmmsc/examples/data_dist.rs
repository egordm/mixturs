use nalgebra::{DMatrix, DVector};
use ndarray::{Array1, Array2};
use ndarray_npy::read_npy;
use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use dpmmsc::plotting::{Cluster2D, Ellipse, init_axes2d, axes_range_from_points};
use dpmmsc::stats::row_covariance;

const PATH: &str = "examples/data/plot/data_dist.png";

fn main() {
    let x_data: Array2<f64> = read_npy("examples/data/x.npy").unwrap();
    let x = DMatrix::from_row_slice(x_data.nrows(), x_data.ncols(), &x_data.as_slice().unwrap());
    let y_data: Array1<i64> = read_npy("examples/data/y.npy").unwrap();
    let y = DVector::from_row_slice(&y_data.as_slice().unwrap());
    let y = y.map(|x| x as usize).into_owned();

    let (mut range_x, mut range_y) = axes_range_from_points(&x);
    let root: DrawingArea<BitMapBackend, Shift> = BitMapBackend::new(PATH, (1024, 768)).into_drawing_area();
    let mut plot_ctx: ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>> = init_axes2d((range_x, range_y), &root);

    // Scatter plot points
    plot_ctx.draw_series(
        x
            .row_iter()
            .zip(y.iter())
            .map(|(row, label)|
                Circle::new((row[0], row[1]), 2, Palette99::pick(*label).mix(0.4).filled())
            ),
    ).unwrap();

    for k in 0..7 {
        let idx = y.iter().enumerate().filter_map(|(i, &y)| if y == k { Some(i) } else { None }).collect::<Vec<usize>>();
        let points = x.select_rows(&idx);

        let mu = points.row_mean().transpose().into_owned();
        let cov = row_covariance(&points);

        plot_ctx.draw_series(
            Cluster2D::from_mat(
                &mu, &cov,
                100,
                Palette99::pick(k + 1).filled(),
                Palette99::pick(k + 1).stroke_width(2),
            )
        ).unwrap();
    }

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", PATH);
}