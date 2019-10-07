package olcar;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;

import org.junit.BeforeClass;
import org.junit.Test;
import org.openlca.core.matrix.format.DenseMatrix;
import org.openlca.core.matrix.format.HashPointMatrix;
import org.openlca.julia.Julia;
import org.openlca.julia.JuliaSolver;

public class BlasTest {

	@BeforeClass
	public static void setup() {
		Julia.loadFromDir(Tests.getLibDir());
	}

	@Test
	public void testMatrixMatrixMult() {
		double[] a = { 1, 4, 2, 5, 3, 6 };
		double[] b = { 7, 8, 9, 10, 11, 12 };
		double[] c = new double[4];
		Julia.mmult(2, 2, 3, a, b, c);
		assertArrayEquals(new double[] { 50, 122, 68, 167 }, c, 1e-16);
	}

	@Test
	public void testSparseMatrixMatrixMult() {
		// currently auto-conversion to a dense matrix
		HashPointMatrix a = new HashPointMatrix(new double[][] {
				{ 1, 2, 3 },
				{ 4, 5, 6 }
		});
		HashPointMatrix b = new HashPointMatrix(new double[][] {
				{ 7, 10 },
				{ 8, 11 },
				{ 9, 12 }
		});
		JuliaSolver solver = new JuliaSolver();
		DenseMatrix m = solver.multiply(a, b);
		assertEquals(m.get(0, 0), 50, 1e-10);
	}

	@Test
	public void testMatrixVectorMult() {
		double[] a = { 1, 4, 2, 5, 3, 6 };
		double[] x = { 2, 1, 0.5 };
		double[] y = new double[2];
		Julia.mvmult(2, 3, a, x, y);
		assertArrayEquals(new double[] { 5.5, 16 }, y, 1e-16);
	}
}
