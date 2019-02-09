package jumf;

import java.io.File;

public class Main {

	public static void main(String[] args) {
		String lib = "../../rust/jumf/bin/jumf.dll";
		File file = new File(lib);
		System.out.println("load " + file);
		System.load(file.getAbsolutePath());
		System.out.println(FFI.sum(new double[] { 1, 2, 3 }));
	}
}
