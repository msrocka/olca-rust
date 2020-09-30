package olcar;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assume.assumeTrue;

import org.junit.BeforeClass;
import org.junit.Test;
import org.openlca.core.matrix.format.CSCMatrix;
import org.openlca.core.matrix.format.HashPointMatrix;
import org.openlca.julia.Julia;
import org.openlca.julia.SparseFactorization;
import org.openlca.julia.Umfpack;

public class UmfpackTest {

	@BeforeClass
	public static void setup() {
		Julia.loadFromDir(Tests.getLibDir());
	}

	@Test
	public void testSolveNative() {
		assumeTrue(Julia.hasSparseLibraries());
		double[] x = new double[5];
		Julia.umfSolve(5,
				new int[] { 0, 2, 5, 9, 10, 12 },
				new int[] { 0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4 },
				new double[] { 2., 3., 3., -1., 4., 4., -3., 1., 2., 2., 6.,
						1. },
				new double[] { 8., 45., -3., 3., 19. },
				x);
		assertArrayEquals(
				new double[] { 1d, 2d, 3d, 4d, 5d }, x, 1e-8);
	}

	@Test
	public void testSolveMatrix() {
		assumeTrue(Julia.hasSparseLibraries());
		HashPointMatrix m = new HashPointMatrix(new double[][] {
				{ 2.0, 3.0, 0.0, 0.0, 0.0 },
				{ 3.0, 0.0, 4.0, 0.0, 6.0 },
				{ 0.0, -1.0, -3.0, 2.0, 0.0 },
				{ 0.0, 0.0, 1.0, 0.0, 0.0 },
				{ 0.0, 4.0, 2.0, 0.0, 1.0 } });
		CSCMatrix uMatrix = CSCMatrix.of(m);
		double[] demand = { 8., 45., -3., 3., 19. };
		double[] x = Umfpack.solve(uMatrix, demand);
		assertArrayEquals(
				new double[] { 1d, 2d, 3d, 4d, 5d }, x, 1e-8);
	}

	@Test
	public void testFactorizeMatrix() {
		assumeTrue(Julia.hasSparseLibraries());
		var m = new HashPointMatrix(new double[][] {
				{ 2.0, 3.0, 0.0, 0.0, 0.0 },
				{ 3.0, 0.0, 4.0, 0.0, 6.0 },
				{ 0.0, -1.0, -3.0, 2.0, 0.0 },
				{ 0.0, 0.0, 1.0, 0.0, 0.0 },
				{ 0.0, 4.0, 2.0, 0.0, 1.0 } });
		var csc = CSCMatrix.of(m);
		var factorization = SparseFactorization.of(csc);
		double[] b = { 8., 45., -3., 3., 19. };
		double[] x = factorization.solve(b);
		assertArrayEquals(
				new double[] { 1d, 2d, 3d, 4d, 5d }, x, 1e-8);
		//factorization.dispose();
	}
}
