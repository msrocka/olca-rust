package olcar;

import static org.junit.Assert.assertArrayEquals;

import java.io.File;

import org.junit.BeforeClass;
import org.junit.Test;
import org.openlca.julia.Julia;

public class BlasTest {

	@BeforeClass
	public static void setup() {
		FFI.load(new File("../bin"));
	}

	@Test
	public void testMVMUL() {
		double[] a = { 1, 4, 2, 5, 3, 6 };
		double[] x = { 2, 1, 0.5 };
		double[] y = new double[2];
		Julia.mvmult(2, 3, a, x, y);
		assertArrayEquals(new double[] { 5.5, 16 }, y, 1e-16);
	}
}
