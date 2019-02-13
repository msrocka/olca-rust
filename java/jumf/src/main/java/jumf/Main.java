package jumf;

import java.io.File;

public class Main {

	public static void main(String[] args) {
		File libDir = new File("../../rust/jumf/bin");
		FFI.load(libDir);

		double[] x = new double[5];
		FFI.solve(5,
				new int[] { 0, 2, 5, 9, 10, 12 },
				new int[] { 0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4 },
				new double[] { 2., 3., 3., -1., 4., 4., -3., 1., 2., 2., 6.,
						1. },
				new double[] { 8., 45., -3., 3., 19. },
				x);

		for (int i = 0; i < x.length; i++) {
			System.out.println("x["+ i +"] = " + x[i]);
		}
	}
}
