/*
 * Copyright (c) 2017, GoMint, BlackyPaw and geNAZt
 *
 * This code is licensed under the BSD license found in the
 * LICENSE file in the root directory of this source tree.
 */
package io.gomint.crypto;

import static com.google.common.io.ByteStreams.copy;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import oshi.PlatformEnum;
import oshi.SystemInfo;

/**
 * @author geNAZt
 * @version 1.0
 */
public final class NativeCode {

    private final String name;

    private boolean loaded;

    /**
     * Create a new native wrapper
     *
     * @param name       of the native lib we want to load
     */
    public NativeCode( String name ) {
        this.name = name;
    }

    /**
     * Try to load the native implementation
     *
     * @return true when the native implementation loaded, false otherwise
     */
    public boolean load() {
        if ( !loaded && isSupported() ) {
            String fullName = "gomint-" + name;

            try {
                System.loadLibrary( fullName );
                loaded = true;
            } catch ( Throwable t ) {
            }

            if ( !loaded ) {
                String suffix = SystemInfo.getCurrentPlatformEnum() == PlatformEnum.WINDOWS ? ".dll" : ".so";
                String prefix = SystemInfo.getCurrentPlatformEnum() == PlatformEnum.WINDOWS ? "" : "lib";
                try ( InputStream soFile = this.getInput( prefix, suffix ) ) {
                    if ( soFile == null ) {
                        loaded = false;
                        return false;
                    }

                    // Else we will create and copy it to a temp file
                    File temp = File.createTempFile( fullName, suffix );

                    // Don't leave cruft on filesystem
                    temp.deleteOnExit();

                    try ( OutputStream outputStream = new FileOutputStream( temp ) ) {
                        copy( soFile, outputStream );
                    }

                    System.load( temp.getPath() );
                    loaded = true;
                } catch ( IOException ex ) {
                    // Can't write to tmp?
                } catch ( UnsatisfiedLinkError ex ) {
                    System.out.println( "Could not load native library: " + ex.getMessage() );
                }
            }
        }

        return loaded;
    }

    private InputStream getInput( String prefix, String suffix ) {
        InputStream in = NativeCode.class.getClassLoader().getResourceAsStream( prefix + this.name + suffix );
        if ( in == null ) {
            try {
                in = new FileInputStream( "./src/main/resources/" + prefix + this.name + suffix );
            } catch ( FileNotFoundException e ) {
                // Ignored -.-
            }
        }

        return in;
    }

    /**
     * Check if the current platform is supported by native code or not
     *
     * @return true when supported, false when not
     */
    private static boolean isSupported() {
        // We currently only support windows and linux x64
        return ( SystemInfo.getCurrentPlatformEnum() == PlatformEnum.WINDOWS ||
            SystemInfo.getCurrentPlatformEnum() == PlatformEnum.LINUX ) &&
            "amd64".equals( System.getProperty( "os.arch" ) );
    }

}
