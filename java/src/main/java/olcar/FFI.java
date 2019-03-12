package olcar;

import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public class FFI {

	private static final AtomicBoolean loaded = new AtomicBoolean(false);

	/*
	 * TODO: when we integrate this into openLCA it would be good to have alse a
	 * single FFI class which contains the native function bindings and the
	 * machinery to load the library public static native void solve( int n, int[]
	 * columnPointers, int[] rowIndices, double[] values, double[] demand, double[]
	 * result);
	 */

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
				"olcar.dll" 
			};
		}
		if (os == OS.LINUX) {
			return new String[] { 
				"libolcar.so" 
			};
		}
		if (os == OS.MAC_OS) {
			return new String[] {
				"libumfpack.dylib",
				"libolcar.dylib" 
			};
		}
		return new String[] {};
	}

}
