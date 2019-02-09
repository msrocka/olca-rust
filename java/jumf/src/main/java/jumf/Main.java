package jumf;

import java.io.File;

public class Main {

	public static void main(String[] args) {
		String lib = "../../rust/jumf/bin/jumf.dll";
		File file = new File(lib);
		System.out.println("load " + file);
		System.load(file.getAbsolutePath());
		System.out.println(FFI.sum(new double[] { 1, 2, 3 }));

		double[] x = new double[5];
		FFI.solve(5,
				new int[] { 0, 2, 5, 9, 10, 12 },
				new int[] { 0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4 },
				new double[] { 2., 3., 3., -1., 4., 4., -3., 1., 2., 2., 6.,
						1. },
				new double[] { 8., 45., -3., 3., 19. },
				x);

		// assertArrayEquals(
		//		new double[] { 1d, 2d, 3d, 4d, 5d }, x, 1e-8);

		for (int i = 0; i < x.length; i++) {
			System.out.println("x["+ i +"] = " + x[i]);
		}

	}
}
