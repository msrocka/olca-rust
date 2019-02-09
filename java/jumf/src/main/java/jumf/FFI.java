package jumf;

public class FFI {

	public static native double sum(double[] vals);

	// UMFPACK
	public static native void solve(
			int n,
			int[] columnPointers,
			int[] rowIndices,
			double[] values,
			double[] demand,
			double[] result);
}
