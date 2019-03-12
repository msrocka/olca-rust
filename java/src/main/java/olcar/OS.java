package olcar;

import java.util.Locale;

public enum OS {

	LINUX,

	MAC_OS,

	WINDOWS,

	OTHER;

	private static OS detected = null;

	public static OS get() {
		if (detected != null)
			return detected;
		String os = System.getProperty("os.name", "generic")
				.toLowerCase(Locale.ENGLISH);
		if (os.contains("mac") || os.contains("darwin")) {
			detected = MAC_OS;
		} else if (os.contains("windows")) {
			detected = WINDOWS;
		} else if (os.contains("linux")) {
			detected = LINUX;
		} else {
			detected = OTHER;
		}
		return detected;
	}

	@Override
	public String toString() {
		switch (this) {
		case LINUX:
			return "Linux";
		case MAC_OS:
			return "macOS";
		case OTHER:
			return "Other";
		case WINDOWS:
			return "Windows";
		default:
			return "Other";
		}
	}
}
