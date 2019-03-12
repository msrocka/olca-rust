package olcar;

import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public class FFI {

	private static final AtomicBoolean loaded = new AtomicBoolean(false);
	private static final AtomicBoolean withUmfpack = new AtomicBoolean(false);
	/*
	 * TODO: when we integrate this into openLCA it would be good to have alse a
	 * single FFI class which contains the native function bindings and the
	 * machinery to load the library
	 */

	public static boolean isLoaded() {
		return loaded.get();
	}

	public static boolean isWithUmfpack() {
		return withUmfpack.get();
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
				LinkOption opt = linkOption(folder);
				if (opt == null || opt == LinkOption.NONE ) {
					// TODO: log something
					return false;
				}
				for (String lib : libs(opt)) {
					File f = new File(folder, lib);
					System.load(f.getAbsolutePath());
				}
				loaded.set(true);
				if (opt == LinkOption.ALL){
					withUmfpack.set(true);
				}
				return true;
			} catch (Exception e) {
				e.printStackTrace();
				return false;
			}
		}
	}

	private static String[] libs(LinkOption opt) {
		if (opt == null || opt == LinkOption.NONE)
			return null;

		OS os = OS.get();

		if (os == OS.WINDOWS) {
			if (opt == LinkOption.ALL) {
				return new String[]{
					"olcar_withumf.dll"
				};
			} else {
				return new String[] {
					"olcar.dll"
				};
			}
		}

		if (os == OS.LINUX) {
			if (opt == LinkOption.ALL) {
				return new String[]{
					"libolcar_withumf.so"
				};
			} else {
				return new String[] {
					"libolcar.so"
				};
			}
		}

		if (os == OS.MAC_OS) {
			if (opt == LinkOption.ALL) {
				return new String[]{
					"libumfpack.dylib",
					"libolcar_withumf.dylib"
				};
			} else {
				return new String[] {
					"libolcar.dylib"
				};
			}
		}
		return null;
	}

	/**
	 * Searches for the library which can be linked. When there are multiple
	 * link options it chooses the one with more functions.
	 */
	private static LinkOption linkOption(File dir) {
		if (dir == null || !dir.exists())
			return LinkOption.NONE;
		LinkOption opt = LinkOption.NONE;
		for (File f : dir.listFiles()) {
			if (!f.isFile())
				continue;
			if (f.getName().contains("olcar_withumf")) {
				return LinkOption.ALL;
			}
			if (f.getName().contains("olcar")) {
				opt = LinkOption.BLAS;
				continue;
			}
		}
		return opt;
	}

	private enum LinkOption {
		NONE, BLAS, ALL
	}
}
