use crate::os::Os;

mod distro;
mod download;
mod download_distros;
mod download_menu;
mod os;
#[cfg(test)]
mod tests;
mod work;

fn main() {
    let aix = Os {
        path: "/etc/oracle/aix.txt",
        x: 125,
        y: 186,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let alpine = Os {
        path: "/etc/oracle/alpine.txt",
        x: 14,
        y: 89,
        z: 129,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let anarchy = Os {
        path: "/etc/oracle/anarchy.txt",
        x: 56,
        y: 55,
        z: 55,
        xx: 27,
        yy: 147,
        zz: 209,
    };

    let android = Os {
        path: "/etc/oracle/android.txt",
        x: 164,
        y: 202,
        z: 57,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let antergos = Os {
        path: "/etc/oracle/antergos.txt",
        x: 69,
        y: 106,
        z: 161,
        xx: 109,
        yy: 138,
        zz: 182,
    };

    let antix = Os {
        path: "/etc/oracle/antix.txt",
        x: 218,
        y: 222,
        z: 226,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let aosc = Os {
        path: "/etc/oracle/aosc.txt",
        x: 0,
        y: 0,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let apricity = Os {
        path: "/etc/oracle/apricity.txt",
        x: 233,
        y: 233,
        z: 233,
        xx: 49,
        yy: 49,
        zz: 49,
    };

    let arch = Os {
        path: "/etc/oracle/arch.txt",
        x: 23,
        y: 147,
        z: 209,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let archbox = Os {
        path: "/etc/oracle/archbox.txt",
        x: 255,
        y: 255,
        z: 255,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let archlabs = Os {
        path: "/etc/oracle/archlabs.txt",
        x: 58,
        y: 58,
        z: 58,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let archmerge = Os {
        path: "/etc/oracle/archmerge.txt",
        x: 103,
        y: 144,
        z: 236,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let arch_xferience = Os {
        path: "/etc/oracle/arch_xferience.txt",
        x: 23,
        y: 147,
        z: 209,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    // TODO Needs more work
    let artix = Os {
        path: "/etc/oracle/artix.txt",
        x: 23,
        y: 147,
        z: 209,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let arya = Os {
        path: "/etc/oracle/arya.txt",
        x: 0,
        y: 175,
        z: 0,
        xx: 255,
        yy: 97,
        zz: 0,
    };

    let blag = Os {
        path: "/etc/oracle/blag.txt",
        x: 0,
        y: 2,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let bitrig = Os {
        path: "/etc/oracle/bitrig.txt",
        x: 255,
        y: 255,
        z: 255,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let blankon = Os {
        path: "/etc/oracle/blankon.txt",
        x: 255,
        y: 255,
        z: 255,
        xx: 246,
        yy: 0,
        zz: 0,
    };

    let bunsenlabs = Os {
        path: "/etc/oracle/bunsenlabs.txt",
        x: 246,
        y: 182,
        z: 32,
        xx: 255,
        yy: 255,
        zz: 255,
    };
    let calculate = Os {
        path: "/etc/oracle/calculate.txt",
        x: 0,
        y: 0,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };
    // Needs reworked for many colors
    let centos = Os {
        path: "/etc/oracle/centos.txt",
        x: 0,
        y: 0,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let chaletos = Os {
        path: "/etc/oracle/chaletos.txt",
        x: 121,
        y: 121,
        z: 121,
        xx: 34,
        yy: 34,
        zz: 34,
    };

    let chapeau = Os {
        path: "/etc/oracle/chapeau.txt",
        x: 71,
        y: 119,
        z: 1,
        xx: 255,
        yy: 255,
        zz: 255,
    };
    let cloveros = Os {
        path: "/etc/oracle/cloveros.txt",
        x: 6,
        y: 105,
        z: 58,
        xx: 28,
        yy: 157,
        zz: 27,
    };
    let coreos = Os {
        path: "/etc/oracle/coreos.txt",
        x: 84,
        y: 164,
        z: 217,
        xx: 241,
        yy: 96,
        zz: 111,
    };

    let crux = Os {
        path: "/etc/oracle/crux.txt",
        x: 37,
        y: 177,
        z: 242,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let debian = Os {
        path: "/etc/oracle/debian.txt",
        x: 215,
        y: 7,
        z: 81,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let elementary = Os {
        path: "/etc/oracle/elementary.txt",
        x: 0,
        y: 0,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let fedora = Os {
        path: "/etc/oracle/fedora.txt",
        x: 41,
        y: 65,
        z: 114,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let kali = Os {
        path: "/etc/oracle/kali.txt",
        x: 82,
        y: 125,
        z: 151,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let kubuntu = Os {
        path: "/etc/oracle/kubuntu.txt",
        x: 0,
        y: 121,
        z: 193,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let lubuntu = Os {
        path: "/etc/oracle/lubuntu.txt",
        x: 0,
        y: 104,
        z: 200,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let mint = Os {
        path: "/etc/oracle/mint.txt",
        x: 151,
        y: 219,
        z: 84,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let manjaro = Os {
        path: "/etc/oracle/manjaro.txt",
        x: 52,
        y: 190,
        z: 91,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let qubes = Os {
        path: "/etc/oracle/qubes.txt",
        x: 124,
        y: 171,
        z: 255,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let rhel = Os {
        path: "/etc/oracle/rhel.txt",
        x: 204,
        y: 0,
        z: 0,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let ubuntu = Os {
        path: "/etc/oracle/ubuntu.txt",
        x: 233,
        y: 84,
        z: 32,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let ubuntu_budgie = Os {
        path: "/etc/oracle/ubuntu-budgie.txt",
        x: 48,
        y: 174,
        z: 250,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let ubuntu_mate = Os {
        path: "/etc/oracle/ubuntu-mate.txt",
        x: 135,
        y: 165,
        z: 86,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let ubuntu_studio = Os {
        path: "/etc/oracle/ubuntu-studio.txt",
        x: 0,
        y: 150,
        z: 241,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let xubuntu = Os {
        path: "/etc/oracle/xubuntu.txt",
        x: 0,
        y: 64,
        z: 173,
        xx: 255,
        yy: 255,
        zz: 255,
    };

    let void = Os {
        path: "/etc/oracle/void.txt",
        x: 71,
        y: 128,
        z: 97,
        xx: 0,
        yy: 0,
        zz: 0,
    };

    let distro_array: [Os; 42] = [
        /*00*/ aix,
        /*01*/ alpine,
        /*02*/ anarchy,
        /*03*/ android,
        /*04*/ antergos,
        /*05*/ antix,
        /*06*/ aosc,
        /*07*/ apricity,
        /*08*/ arch,
        /*09*/ archbox,
        /*10*/ archlabs,
        /*11*/ archmerge,
        /*12*/ arch_xferience,
        /*13*/ artix,
        /*14*/ arya,
        /*15*/ blag,
        /*16*/ bitrig,
        /*17*/ blankon,
        /*18*/ bunsenlabs,
        /*19*/ calculate,
        /*20*/ centos,
        /*21*/ chaletos,
        /*22*/ chapeau,
        /*23*/ cloveros,
        /*24*/ coreos,
        /*25*/ crux,
        /*26*/ debian,
        /*27*/
        elementary,
        /*28*/ fedora,
        /*29*/ kali,
        /*30*/ kubuntu,
        /*31*/ lubuntu,
        /*32*/ manjaro,
        /*33*/ mint,
        /*34*/ qubes,
        /*35*/ rhel,
        /*36*/ ubuntu,
        /*37*/ ubuntu_budgie,
        /*38*/ ubuntu_mate,
        /*39*/ ubuntu_studio,
        /*40*/ xubuntu,
        /*41*/ void,
    ];
    let neofetch = &work::neofetch();
    // When adding distros adjust
    // distro.rs line 11
    // download.rs line 30
    // download_menu.rs line 7
    // download_distros.rs line 120
    // os.rs line 27
    // work.rs line 267, 242, 246
    distro::distro_menu(neofetch, &distro_array);

    // NEED Colors for aix, alpine, anarchy, antergos, antix, aosc, apricity,
    // arch_xferience, archbox, archmerge, archlabs, artix, aryya, bitrig, blag
}

// My Notes
// Try making build.rs that copies files (ascii art) from a folder (ascii) inside the src directory

// Message Author of progress bar about hexadecimal color, or copy his crate with ability to use hexadecimal color

// Download Menu should print columns dynamically in relation to available space in terminal

// EXPERIMENT
// writing iso file as bytes to usb
// could use same progress bar as download

// Options
// -d,  max depth search for iso, img
// -n,  don't run neofetch
// -D, download only

// ihalila/pancurses
// jeaye/ncurses-rs
// immunant/c2rust

// look at https://github.com/redox-os/coreutils/blob/master/src/bin/dd.rs
// for ideas to replaces dd with pure rust

// look to https://github.com/redox-os/netutils/blob/master/src/ping/main.rs
// to solve no internet fail issue
