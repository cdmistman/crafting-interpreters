package lox;

import java.io.IOException;
import java.io.PrintWriter;
import java.security.MessageDigest;
import java.util.Arrays;
import java.util.List;

import org.gradle.api.Plugin;
import org.gradle.api.Project;

// public abstract class GenerateAst implements Plugin<Project> {
// 	private MessageDigest digest = MessageDigest.getInstance("SHA-256");

// 	void apply(Project project) throws IOException {
// 	}

// 	private String checksum(String contents) {
// 		return digest.digest(contents.getBytes());
// 	}
// }

// abstract class GenerateAstExtension {}
