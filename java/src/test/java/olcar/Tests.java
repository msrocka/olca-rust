package olcar;

import java.io.File;

class Tests {

	private static final File libdir = null;

	public static File getLibDir() {
		if (libdir != null)
			return libdir;
		String path = System.getProperty("olca.libdir");
		File dir = path != null
			? new File(path)
			: new File("./bin");
		if (!dir.exists() || !dir.isDirectory()) {
			throw new RuntimeException(
				dir.getAbsolutePath() + " does not exist");
		}
		return dir;
	}
}
