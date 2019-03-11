package jumf;

import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public class FFI {

	private static final AtomicBoolean loaded = new AtomicBoolean(false);

	public static native void solve(
			int n,
			int[] columnPointers,
			int[] rowIndices,
			double[] values,
			double[] demand,
			double[] result);

	public static boolean isLoaded() {
		return loaded.get();
	}

	public static boolean load(File folder) {
		if (loaded.get())
			return true;
		if (folder == null || !folder.exists())
			return false;
		synchronized (loaded) {
			if (loaded.get())
				return true;
			try {
				for (String lib : libs()) {
					File f = new File(folder, lib);
					System.load(f.getAbsolutePath());
					loaded.set(true);
				}
				loaded.set(true);
				return true;
			} catch (Exception e) {
				e.printStackTrace();
				return false;
			}
		}
	}

	private static String[] libs() {
		OS os = OS.get();
		if (os == OS.WINDOWS) {
			return new String[] {
					"libwinpthread-1.dll",
					"libgcc_s_seh-1.dll",
					"libquadmath-0.dll",
					"libgfortran-3.dll",
					"libopenblas64_.dll",
					"libsuitesparseconfig.dll",
					"libcolamd.dll",
					"libamd.dll",
					"libcamd.dll",
					"libccolamd.dll",
					"libcholmod.dll",
					"libumfpack.dll",
					"jumf.dll"
			};
		}
		if (os == OS.LINUX) {
			return new String[] {
				"libgcc_s.so.1",
				"libstdc++.so.6",
				"libquadmath.so.0",
				"libgfortran.so.4",
				"libopenblas64_.so",
				"libsuitesparseconfig.so",
				"libcolamd.so",
				"libamd.so",
				"libcamd.so",
				"libccolamd.so",
				"libcholmod.so",
				"libumfpack.so",
				"libjumf.so"
			};
		}
		return new String[] {};
	}
}
