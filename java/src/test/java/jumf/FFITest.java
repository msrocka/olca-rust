package jumf;

import static org.junit.Assert.assertArrayEquals;

import java.io.File;

import org.junit.Test;

public class FFITest {

	@Test
	public void test() {
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

		 assertArrayEquals(
				new double[] { 1d, 2d, 3d, 4d, 5d }, x, 1e-8);

	}
}
