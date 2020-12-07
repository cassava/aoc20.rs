/*!
# Day 2: Password Philosophy

Your flight departs in a few days from the coastal airport; the easiest way
down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
"Something's wrong with our computers; we can't log in!" You ask if you can
take a look.

Their password database seems to be a little corrupted: some of the passwords
wouldn't have been allowed by the Official Toboggan Corporate Policy that was
in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of
passwords (according to the corrupted database) and the corporate policy when
that password was set.

For example, suppose you have the following list:

    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy
indicates the lowest and highest number of times a given letter must appear for
the password to be valid. For example, 1-3 a means that the password must
contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is
not; it contains no instances of b, but needs at least 1. The first and third
passwords are valid: they contain one a or nine c, both within the limits of
their respective policies.

How many passwords are valid according to their policies?

## Part Two

While it appears you validated the passwords correctly, they don't seem to be
what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the
password policy rules from his old job at the sled rental place down the
street! The Official Toboggan Corporate Policy actually works a little
differently.

Each policy actually describes two positions in the password, where 1 means the
first character, 2 means the second character, and so on. (Be careful; Toboggan
Corporate Policies have no concept of "index zero"!) Exactly one of these
positions must contain the given letter. Other occurrences of the letter are
irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
*/

use aoc20;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

fn main() {
    let mut input = aoc20::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 2: {}", PUZZLE);

    let policies: Vec<(PasswordPolicy, String)> = input
        .to_str()
        .lines()
        .map(|x| {
            let mut pair = x.split(": ");
            (
                pair.next().unwrap().parse().unwrap(),
                pair.next().unwrap().to_string(),
            )
        })
        .collect();

    println!(
        ":: Number of valid (sled rental) passwords is: {}",
        policies
            .iter()
            .filter(|(policy, pass)| policy.is_valid(pass))
            .count()
    );

    println!(
        ":: Number of valid (Toboggan Corporate) passwords is: {}",
        policies
            .iter()
            .filter(|(policy, pass)| policy.is_valid_v2(pass))
            .count()
    )
}

#[derive(Debug)]
pub struct PasswordPolicy {
    c: char,
    min: usize,
    max: usize,
}

impl PasswordPolicy {
    pub fn is_valid(&self, s: &str) -> bool {
        let count = s.chars().filter(|c| *c == self.c).count();
        count >= self.min && count <= self.max
    }

    pub fn is_valid_v2(&self, s: &str) -> bool {
        let chars: Vec<_> = s.chars().collect();
        if chars.len() < self.min {
            false
        } else if chars.len() < self.max {
            chars[self.min - 1] == self.c
        } else {
            (chars[self.min - 1] == self.c) != (chars[self.max - 1] == self.c)
        }
    }
}

#[derive(Error, Debug)]
pub enum PasswordPolicyError {
    #[error("invalid data: {value}")]
    Invalid { value: String },
}

impl FromStr for PasswordPolicy {
    type Err = PasswordPolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z])$").unwrap();
        }

        let captures = RE
            .captures(s)
            .ok_or(Self::Err::Invalid { value: s.into() })?;
        let capture = |idx| captures.get(idx).unwrap().as_str();

        Ok(Self {
            c: capture(3).chars().next().unwrap(),
            min: capture(1).parse().unwrap(),
            max: capture(2).parse().unwrap(),
        })
    }
}

const PUZZLE: &'static str = "Password Problems";
const INPUT: &'static str = r"
5-9 g: ggccggmgn
11-16 l: llllqllllllllflq
3-6 q: qvqqqpzqd
6-11 f: ffffpcffffp
7-8 l: lllqlwtqll
8-10 q: qqglqqqqqqqjqmdbq
4-6 w: hflwkwplswcnhb
6-15 w: wwswwxlwwwwwwww
3-5 f: fffvnpvf
6-8 b: bhwsshbp
1-2 l: lhll
5-6 s: wpsxsss
2-4 w: wswwtwww
9-12 x: xxxxxfxxxxrnx
9-10 s: sssscsqssshsppzsc
1-3 w: wgwtwbksqrrcpvtfbbw
2-9 r: wrrtcmzcrdtlmkldsckq
4-6 g: gttdhlfgpcgrgtwc
1-2 f: fpbcjczfwmrrxk
9-10 z: zzzzzzzzzt
1-8 m: mjjlsbmg
5-7 p: ppkpprp
3-6 w: wwwtwzwww
17-18 q: cqqjvhqhrpqlqvqdjlbb
2-3 n: dnphwnfz
3-4 r: blqrrrr
4-5 v: lsrvvpghqsdwrvppsm
5-15 n: hclnngnhmfnjzqnvz
6-7 d: kddmddd
15-17 v: vvvvvvvvvvvvvvpvl
3-4 q: kndwvq
12-19 t: tktxcppccjlwnnmmntt
6-8 p: ppplpcpc
3-6 p: npprhpsxkq
4-6 v: bvfvvv
1-2 r: rrrr
6-10 r: bsrcqbtrpbr
7-8 w: wwwwwwwvw
12-17 h: hnffmvtrqnmhgsnbhrr
9-13 c: cccjcddcchccc
11-13 c: cwccctccccccc
6-8 d: khjjvdddpkddcddd
1-2 r: lmrr
1-5 t: wtttmt
6-7 m: qdmmmmmxm
5-18 h: hhhhshhhhhhhhhhhhhh
4-11 l: llllllllllcllll
2-8 k: kkbkkkkkkkkk
3-6 q: rqbqmqwqgq
3-6 d: dsrdfh
16-20 m: qmllmddfldjmmrpmmwpt
9-16 j: tjjpjqvhjjjtjjtjj
4-9 b: btbdbbbbrbbb
5-16 w: wwwwpwwwmwwwwwwmwww
1-2 g: gggggbwgggbgg
1-2 l: lllllbcwlllkfb
4-5 h: ghhshkhhhdbs
4-5 s: ssssssvssspssxs
7-12 k: kkpkkkgtrkkk
11-16 t: tgjttttktrtstsxtm
9-13 c: cccccccccccccc
5-9 q: qqqqqqqqq
2-4 q: rqqqq
4-6 l: lllblql
2-4 b: jbbbnvbx
4-9 q: kqqqbxqsq
3-4 q: rzjrcq
1-5 l: rllqsc
1-6 l: lgljslmsmbwbllsjw
2-5 p: psmkj
1-3 v: vvvvv
7-10 k: kbmdkkdkpzxhkqk
3-6 g: ggtgpgxg
1-8 w: wswwwgww
2-6 r: rrqmfs
5-6 r: rrrrpzr
2-5 v: vvfvvv
5-10 r: rdrrrrrnrr
1-7 v: vvvvdvvvv
6-7 r: nhbxnrr
1-4 g: mggr
2-3 g: ggqtgw
7-8 k: qkkkckkkxxwkvt
2-4 g: gbmgx
7-14 w: wrwnftwrbcbxrx
7-10 f: ffkmfftlnf
10-13 j: qjjjjjjjjgjjq
11-13 n: ntnnnnnnnnknsn
1-7 d: lsddrwzzddhxddq
12-17 q: qjqqqrlqdqqvqwqqd
2-3 z: crzz
3-4 d: dtsd
3-5 k: hkgstlvxr
18-19 d: hswndtgrdtwtdddddpl
9-12 x: xxxqxxxxxxxjhxx
1-2 b: dpbb
8-15 h: bhhzkqhghhtphfzh
13-17 d: dnddddddddddddddxd
15-17 m: mmmmmmmmmmmmmmmmmmmm
5-11 r: rrrrwrrrrrwrr
4-6 v: vvvhvf
7-10 l: lllllltxlfllfll
3-7 s: sssssssss
2-7 b: fbfcfdgtbb
4-5 x: xmxxxxx
2-5 g: gqqsqbmgclq
3-5 z: zbzgznzp
7-8 z: hzzxtzzz
2-4 q: qjqqqq
17-19 g: ggggggggggggggggggwg
7-10 s: pwqssvsqmsbfrbfg
8-12 b: bbbbbbbsvbfsfgvbzbk
1-9 n: mnnnnnnnnn
2-3 z: zcwzz
6-18 x: xxxxxtxxxxxxxxxxxlx
6-15 m: mmtmmmmmmmmmmmmdmm
4-8 v: bfbvlmwgrvlh
16-18 m: mmmmmmmmmmmmmmmjmd
8-9 j: hjqdjjjgj
2-3 q: qfkv
10-12 p: kpzvlppqzmpdd
3-4 s: ssss
2-12 c: mcnscktznndc
3-5 l: lllllll
3-4 w: wwww
9-10 s: sssssssssls
11-13 d: dzdddddjddtddw
6-7 x: xxjxxxxdx
7-8 f: dqjhfqfw
8-13 m: mmmmqmmlmmmmcmmmmmmm
7-10 j: bdcmkjjhfjzbfwxgzg
3-8 j: tgmdwwjjnjgjgjhjtx
4-7 l: llllllllvplglld
11-15 z: tzxvzzzzzzmfzzczz
3-6 z: zzzzpzzsgztcpzz
9-13 x: pfzxpqtckxvsxx
7-8 g: gmglbpgxwglxbztbkgcv
3-5 b: vqnbbb
6-7 g: ggggggg
4-10 n: nnqnnnnnnmhqnnnn
7-9 b: bbbbbbqbdbkbqbbb
15-17 h: hhhhhhhhhhhhhhdhjz
2-3 b: bbbxb
1-3 b: bpbvbbbbbbbbzlbrsb
9-13 c: ccccccccpcccvc
10-13 n: hnnnwbnnnntnnn
6-12 s: zssscssfsssszs
2-9 n: pnwkdvknxhvvqpjrqk
1-2 c: hvfdcltktc
3-6 c: cpccblcrqnlfdnd
12-14 h: mxpghswhdlklnb
4-5 b: kbbggvbbxkbsl
1-6 j: chjjjxjjjj
4-6 m: mmmmmmm
13-15 m: mmmmmmmmmmmmrms
9-15 z: zzzzzzzzxzzzzzzz
5-11 q: gqmlbsgjbzv
12-15 t: sgtgscntdttttjt
13-15 h: sgxqbhhhmkfzhmd
7-9 p: dppzplppppt
2-6 v: rzlgcxspgwdwrkxbsqvw
2-5 n: mnlxnnn
17-18 p: pkcjklpppxdqptpnfxk
10-11 s: sssjssssstd
8-11 f: fffffffffffffffff
1-3 h: hhhhhhh
4-5 d: vddxnd
2-10 h: hxhhhhhhhhhh
11-13 c: vrmccccccqcccccchnk
8-14 l: llrlllllllllqljjm
9-15 g: bkglbmgfgkfxnblg
14-16 c: ccccbccccccccccccccc
3-5 l: llglx
1-7 t: qtttgtn
6-8 j: jjjjjjjjkjjjj
11-14 n: btnndcxnbnnjjnnkvn
4-5 r: rgrrq
7-9 l: rxltlljllll
1-5 w: wwrgn
14-18 d: zlxdglmbsstdbqddnq
1-18 m: mmmmmmmmmmmmmmmmmm
13-16 p: prpphpqppplppzpnpppp
3-6 s: nfdsssssss
11-12 b: bmbbstbbwbbt
5-6 x: fstlxx
7-11 w: hwmxkpwwdfmr
2-4 m: lwsrm
11-15 h: hhhhhthhhhhhhdh
11-15 h: hhspzhhghhhhhph
12-13 g: ggggggggggglv
1-2 r: rqgrtrjrrrr
2-5 m: rmmmmmmm
2-12 w: swbdnqjpgkhlgfcwhjtl
3-12 b: bbpbdbbbwbdbb
8-10 z: zzzzzzzzzzzzz
5-12 m: mmmzwmxmmjnhlmdbmm
7-9 q: pqqqqqqqqqqdqq
12-14 z: zmrzqkzmzzwxptz
1-10 n: nnnnwnnnnnnn
2-4 s: gvsw
2-5 f: tbfffff
7-11 p: xpgzpppmppp
6-15 v: jvgvmzvxvvvvvwnvv
1-2 n: nnnnftznb
7-13 t: ttttttttttttttt
19-20 p: pppppppppppppppptplt
1-5 h: xhvbhhpzthh
8-10 b: bbbbbbxwbbbbbbb
7-13 w: wwhbwwwwkwdww
2-4 p: zpppshgljlgzcsp
7-8 p: pngtdrbp
10-12 f: ffffbffffffffff
2-5 t: vttgtqxn
3-4 r: rrfr
4-6 r: rrrrrrr
3-15 c: cnckhgxpwhcgnzw
4-7 d: ldddxzjngg
9-10 b: bgjpmqmjgb
4-5 v: vqhvvvz
9-12 b: wvdsqhdhdcbt
2-3 w: wwww
16-17 w: dwzwwvrjlwlwwnfwlsts
4-5 x: bmbmxpxrg
3-6 p: pjpxbp
1-5 j: vdsjxj
2-3 x: xxsxfxxxxxs
9-15 j: lbwqpstwjjbwpzjkgj
10-13 n: hnvnnnpnnnnnnnnnnnn
10-11 m: tmmmkmmmzmnmvm
6-12 l: lslllllmlllrlxl
8-10 r: rrrrrrrrrrrrrrr
9-11 b: hmhnzhflblcbsrfsj
10-13 h: nhmhwhbhzkrlwh
12-14 s: dsrssjhrsssgsxksssg
2-7 d: nhdlpddffdgdp
1-5 p: pppppp
11-16 k: gcxckpvrtvkhrdkht
9-15 m: lcmmmmkmpmmmmmx
5-6 z: zsrxwzfz
3-4 v: hdvtqqkvz
13-16 w: wwtwlwwnwwwwfwztz
9-11 f: pfwdpfrpfmfmb
4-8 x: wxxxbtzxmrx
9-11 m: mlvmmmzmmmmjnmpkkhm
9-10 w: scpljvzpwnwcbhzntbsw
3-8 h: bgbhhnhndcthhv
8-17 x: xxxxxxxxxxxxxxxxxxxx
5-10 d: dtddddddddd
2-3 v: vvvv
13-14 z: hmrbhfgmdkljtz
5-7 h: hhhtpdd
3-6 w: mwwglwwwwptz
3-4 q: sqqq
12-15 v: vmvvvrvlvvpqvqk
4-9 n: dndmktskr
18-19 d: dddddddmdddddcdgddd
4-5 t: ttttstt
11-17 p: ppppppppppkpqpppvpp
7-11 l: tslnxvldrfhp
14-15 x: xxxxxxxxxxxxxrcxxxx
2-8 b: bbbbbbbbbbbb
12-17 n: nnsnnnnwnpnfnnnnnnnn
3-8 f: crfpffhf
17-18 j: jjwcmkcdxjjjkqtqjj
2-4 n: nnnn
7-9 q: rbrkbqqtj
2-3 j: jstj
18-20 d: dddddddddddddddddddh
7-14 h: hhhhhhxhhhhhhdhh
3-10 g: dgbgpgggxg
11-12 w: wwwwmvqmwlwwj
4-11 s: csrldnzsvhxcc
6-8 n: nnnnnnnnnn
2-5 x: xxxxxld
7-9 m: mkmmmhcbmrj
9-13 p: bppppppppppppppp
3-20 g: ggggnggggggggggvgggt
10-11 d: gddrddddddd
5-10 m: mmmmmmmmmmmmmfmmpm
5-7 n: nnrnccm
11-14 k: khkkknzkrkjkkb
2-10 x: jxrbgfkgjz
19-20 t: tttttttttttttttttkch
1-4 d: vvqfxvrtxldzcmxxdlsj
8-13 n: nnvnnngnnnnnwnnnnb
5-9 n: nnnntnnnwn
2-3 z: zzzz
2-12 b: zblxddslcnbbpcj
5-17 z: zzzzzzzzzzzzzzzzzzzz
14-16 l: pdlllfllllsrgwwf
6-13 j: knwhbnjxwgrwrbjqwcj
3-4 r: rrfwr
2-3 h: hhrh
3-4 l: klll
5-7 b: bqwbbfmbb
5-11 s: shsszsssksssscsssss
6-8 w: mwdbwwgw
2-8 z: tzcqzszzzmq
2-12 f: fgtffwfkxffffffrjsf
4-5 s: sslsk
9-13 s: vsssfsbsssvgzftcss
1-3 g: gkggzr
8-13 d: rchmzwxdkzgpdzwdk
18-19 q: qlmvnrkklmdwrjprzsmv
1-7 r: rrsrrrr
17-19 d: dddddddndpbdwskdddd
7-8 r: rrrrrrcvr
1-3 r: jdrrhq
5-6 r: rwrjtrcmkrrk
2-3 m: mmxmg
1-19 f: ffffffffffffffffffpf
16-17 h: hhhhhhhhhhhhhhhhh
10-13 n: nnnnnnnnnnnnvsn
3-4 g: gggg
7-13 c: ckcvfscqtmckc
4-5 z: zzzdnhzzzf
5-6 n: lznnnnglnk
4-12 q: fjpqwvlbgzqmqmmgdjp
3-4 d: ddldd
5-7 r: cdrlrnrgmtrrrrnr
15-16 j: ppwjfxzqdtpjwwjjndkm
11-12 g: ggggggggggplg
3-4 n: dnjqzsrtc
7-8 t: ttntttbb
4-15 t: tttjtttttttqttztd
5-8 r: rrrrrrrrr
4-5 q: qqqqq
5-7 n: nsvgnnns
12-13 w: wwwwwwwwwwwwwwww
15-17 n: qxtnnnnrcncnmpnnn
3-13 z: mcmvkthxwzkqzglqhfnw
5-7 d: ddddddrdddddd
16-18 m: mmmmmmmmmmlmmmmmmpkk
9-12 l: lllvlnlwlmll
5-12 d: ddxdxdddjjlsddddmpdd
9-18 p: ppppppppbpppppppprpp
1-5 v: lvwpm
7-8 v: hvzvhrvh
9-14 z: zzmzzzzzzzzzlzgzz
3-4 x: wxwtb
2-4 f: fhfszf
2-7 c: chcccctcdccccc
2-12 h: kjctchvdrmlpxqjnqd
3-5 z: pzkbz
3-10 g: ggggpggggsggggg
6-7 c: cccccgbc
3-4 d: ddkjrd
2-3 r: kgwt
8-10 z: mzzzjnzgzzzzzw
12-13 t: tttnztrvttqtttcrht
9-10 x: xxxxdxxxbxxxx
2-4 d: mgdd
1-4 b: bbzb
14-16 b: bsbbbbbbbbbbbkbbb
3-10 n: xnngwgptznfr
16-17 k: kkkkkkrkkkkkkkkrzk
17-18 g: ggtgggggggggggggpw
12-16 j: jjjjjjjjjjjsjxjjj
12-15 f: ffffffffffffffffff
5-6 m: kmmnlmmnm
5-7 j: jbjjlzj
2-3 l: llll
9-12 l: lqzlxljfclwz
14-15 n: nnnnnnnnndnnnsnnn
3-10 l: rpdldkdkpt
4-6 z: mgzzzthzn
4-13 m: jmxtwbnhgmtqggrm
7-8 n: mnnnnnrwnn
5-10 q: qqqqcqqqqqq
1-2 t: kttttt
5-8 b: bbbbbbbbb
4-14 z: zbbffzzlrzzzztzj
6-10 k: kkkkkbkkkk
9-10 j: djjjjjjjmp
4-6 q: qqqwqq
16-17 t: tmtbtcktgtknttkhttq
13-15 x: xxxxcfqxxxxxxxxxx
2-3 f: fzfff
1-3 w: vtlk
9-11 l: lllldzllrbb
9-13 b: zvbbsbbtbbxzp
9-10 v: vvbwvvmvvvvvzznvbv
11-12 w: zwgwhrwpwzlq
1-2 k: cjkk
6-10 b: bbbbbbbbbkrbbbbb
1-3 z: kzvxz
2-3 h: khhh
3-4 h: hhhfdhhh
8-11 l: glllglcxgbwxdfdlp
2-10 j: jsjjjjjjjcjts
2-8 b: bbbbjmbbbbbbbbbbbgbb
1-3 j: cjbtvwm
5-6 f: ffffks
13-14 r: rrrrrrrrrrrrrrr
11-14 v: wpddkmvbgwgttk
4-5 l: pllll
9-10 x: wxxxksxxqxxxwvcqxqw
5-10 h: chgkkkcxgmmqclmxvjs
1-2 x: xsqn
5-10 d: ddxdddkddndddd
2-4 g: jsdggjzvs
2-3 x: xvkx
3-5 b: blbbsb
5-6 r: rrrtrm
12-13 z: rzzjzztpzzszzznb
12-18 h: hmdlhtwrjrlkgzgbkn
6-20 q: fqhwcqqmqqqtqqsqqqkq
1-5 s: shfth
3-12 t: jttjvtdlptrr
4-9 p: ntppvcsxpspr
2-5 j: bjbljfj
4-9 m: mmmwmhmsrmmgnn
4-5 s: vssdg
6-11 p: zpvpppxpvbpjnp
4-5 x: xxxvxx
5-8 x: nxxxxxxlx
3-6 l: lkxnlwwclf
7-8 s: sssssntc
19-20 k: kmkkkkkkjkkzbkjkkkkk
3-4 d: dpjp
3-9 s: lvwsssqss
1-2 c: ccssqkdmxwcmv
7-13 q: qfjqrcqqdqqqn
5-13 x: cqddxmvjwxfnx
1-5 l: lptcl
4-5 h: hvhpt
5-10 g: cggvwnhggggg
2-4 v: nqxvrzmwjbmqnvrb
1-4 t: qtwqrtbssqgtt
9-10 s: ssssssssqf
4-9 v: xwvwvzfwnkvnv
6-7 q: qbpwcqq
2-10 n: nnnnnnnnnbnznnnnnn
5-9 v: vvvvmvvvvv
9-10 l: llllllllll
11-15 m: mmjmxmmmpmxmmmrmmm
1-5 r: hrrrrbr
10-13 g: pkgrbchtpxzrfw
10-11 v: vvvvkvvvvvvf
1-7 p: pprpqpp
1-5 d: dxtqddw
2-4 q: qpqjqqqlqqqqqh
16-17 h: hhhfkhhhhpzhhhhhh
3-6 l: xxclpflwllzh
5-7 z: ztzzzzb
15-16 c: ccccccccccccccqtc
10-11 k: kkkvknkkkkkkk
3-9 n: mnnxxvbhj
6-12 t: tmxlxtxgpnwtkxmk
3-9 k: jkkslslkvb
9-17 n: nnnnnsmjxjgnnhkmn
1-2 c: ccpwch
6-10 p: lfbwlkmzlnxdpsjpsg
9-14 c: cckctccccccccv
12-13 n: nnnnnnnnnnnnnmjn
8-10 n: nnbnnnnjnbnnnnnnnnkn
14-17 h: fkgnhvwvhxrhzhnsh
9-11 w: wwwwwwwwwxwwwwwwww
5-9 q: qqqqqqqqqq
12-13 f: fdfhffmfffffq
4-10 q: wwkqqbttbr
9-10 r: ndrmprfjqf
6-13 h: hhhhhhhhhhhhrh
14-17 t: tkmgwtztkfjtgtttt
2-6 t: vqtcptqrbrs
2-3 r: rrtrrrrrr
3-8 j: nhjtkjhjwlf
5-17 b: vbtctpbhtjtkhtwcw
5-8 z: zsbkkcgqwqqvzr
4-8 c: cpccjpwccccnc
6-13 k: rhkghtnkkbqdz
8-11 p: pcppmwppnvp
12-17 k: jjbsfkkqbbqkkknkk
4-6 h: tddhcllhhhcbwq
3-4 q: sqqqqjqm
4-7 b: tbqwbbqcs
3-10 z: zjzzzzzzzhzz
4-5 m: mhxpzwrcxtgsjxfdq
17-18 q: sqqqqqpbqqqqwqhvsp
3-4 n: njnww
9-10 v: vvdvxvvjvv
2-8 k: fkztgksk
5-17 q: fxvjhclcfkcxwhqqk
3-5 p: rbkxlkfzxvpgcf
2-4 f: wlmv
9-10 b: bzxwbbrwhfbbbb
4-10 d: dddbdddddddd
7-8 b: hzrsbxjbhql
5-9 r: rsjrrbrrlrhmrbcvnbr
6-7 b: bbbbbbbf
14-20 m: ddfhmsxmcbwmmfvmmmmf
2-9 m: mmmvmmmmxmmmmmmmm
9-11 q: wqnttwfdqqjhz
18-19 g: ggggggggggggggggggg
13-15 f: fffffffffffdxtffr
2-7 b: crskznfmbjmch
2-4 c: lcccz
2-6 n: thxnnjnnnnnnn
3-6 h: hwhgdhhhh
15-17 d: dddtddddddkdddddvs
4-7 f: zzptbfkrvzcrxflnp
1-2 m: mmhm
2-3 w: pmww
1-3 f: wfkf
6-8 m: rcpxtmmm
6-9 c: cccccccccc
1-3 j: bjjj
4-6 d: dddhddddd
1-6 l: nllllkll
1-2 x: xjnhzxxxpx
3-13 r: rrkrrrrrrrrrjr
18-19 h: hhhhhhhhhhhhhhhhhhhh
7-9 d: dlvrddbvhdddd
6-11 r: rmvqqrxsjxgvkrrskfp
9-10 k: kkkdkkkkkkk
9-10 v: pcjpkrgqvp
1-3 z: rcqfgzwxgrvml
2-9 j: gjbjvgtqjkxbsrvb
5-7 q: qqqvqqrvqqntkbpj
2-3 h: cgnhphpf
7-10 f: ffffffdcfsfffff
4-7 r: brgkrrrrrrrrrrmrrx
4-7 p: pppwppbfpp
7-9 v: vrlvbtlmv
12-13 x: cxxxxjxxxsvxx
3-5 z: lzzxd
4-6 q: qqqqqqxqf
11-12 h: hhmhhhhhhzhh
15-19 k: kkkkkkzkzkkkkkkkkkmk
8-10 p: pmpppnmppf
5-7 m: mtmmmzlmmmm
6-9 d: gsdhmqqldd
4-11 c: pcrrwnbtckcx
6-12 g: qhjmzgwbhgmgjg
4-9 w: ntwwcgtwt
1-3 k: knkdkdwpg
5-7 l: qslglll
11-15 p: njpkdhkfdgmjdtp
2-6 p: kcvlss
4-13 f: psffmdwqkqlffw
1-17 g: qgggggggglggggggf
9-10 f: fffhfffftp
18-19 r: fwmkgpdvhsmjrzqqtjq
9-10 b: bbbmbbbbvmbbbdb
8-11 n: mndnrlnnnnn
14-16 h: hhhhhhhhhhhhhthh
2-3 l: lztsl
2-4 d: djtp
4-6 f: mpmfzt
2-4 h: tnhh
3-4 d: dcdm
4-10 b: kzblbhccnbkb
2-6 d: ftbdjzns
10-13 l: mllllllllglllllll
11-15 n: nnnntnnnnnjrnnnn
11-13 b: tlsblhbjfbpbrb
2-6 x: fwwxtxw
2-3 t: ttgttttttttttttttttt
6-10 f: gffffffflf
3-4 z: zzzz
5-15 d: rnmfddgdvddvxkdp
6-9 b: bbbbbbbbbb
1-4 k: kkskk
1-3 b: bbbbbbbbbbbbbbbbbb
1-5 b: wbbbpbqbsb
1-4 g: xgfg
1-13 d: ddddddddddhdddd
6-7 p: ppppppw
13-15 b: bbbbtbbbbbpblbb
3-13 h: hkhnkkfkrwrbz
5-6 r: rrrrvn
2-6 w: wwwwzwwvwwzjnrwwwwww
1-4 h: hhhh
4-10 d: dsndddptsdqqgzjlgrwd
4-5 l: glmvll
3-4 c: chwcc
2-7 m: mpmmmmm
1-2 x: xxqxwm
7-8 m: mmmmmmmmm
5-8 h: hbhhwhkhqmkhh
3-4 g: gwggk
4-5 p: pmppp
10-15 p: jcppppphvpvsqlpcdb
2-8 w: wwvkzvmkwrxh
18-19 h: jptsvhzvbnhbghmhghgn
4-8 s: dhrscshhxczcfqdsm
8-10 q: qqqqqqqlql
13-17 c: lbmrzdcjcnwkclgcc
12-13 w: wwtwwwwmrcrkwwwwwww
5-16 j: jvqzfwsnqfdxfzdjj
1-3 h: ddzmhh
4-13 t: tttttttpctctrtztzqn
3-4 z: zzzz
1-18 w: txwwwwkwwwwwwwpwwwww
5-14 l: cvrnvmrzlmkxgllr
9-10 j: jjjjjjjjqv
13-17 g: wgglspgdgxgpspbgwn
13-15 s: sssssssssssssss
8-9 v: mlxvtvjnt
3-7 s: dgdjqnslcvsvdwsdfkvf
2-3 q: mgqd
2-3 x: xxstwczx
2-6 p: nqqjzskbxzg
4-5 g: qcxggbrnjzgdtq
9-16 p: ppppppppcnbpvppphppr
7-11 j: jjjjjjsjjjk
3-6 n: cnnnnk
3-6 l: glcllrllll
2-3 s: scmxssssssssd
8-16 c: cccfcccccccclbhcccc
1-9 f: qfktgmfsgffqmsfsqp
2-6 l: lhltlzlllqlwllzl
5-10 b: flphbpspbbsbjnjf
4-5 q: qbqcqq
2-3 f: fgffstff
3-10 w: wwwwwwwwwwwww
11-12 d: ddddsdddddwkd
6-9 h: hhhhfdqhhhq
13-14 z: zzzzdxszzzzzzgzzzz
3-4 l: lljjcnrlwsg
7-9 c: jccsccccccc
1-13 k: gkkkkkkkkkkksk
6-13 w: wwwwxwwwwhwfrwwww
5-6 z: zzzhjczzzzz
4-5 t: ttttt
5-9 j: jjjfjjjkkjjz
5-11 l: lfzlxtbnlhrdl
16-17 l: llklllllllllllllsl
2-5 b: bbbbbbbd
8-10 l: llllrllxld
4-14 t: vxgtxxcvrlspxw
2-4 b: jkhv
10-11 f: zfffffffwkx
3-4 w: wvvr
6-14 h: rlcvzhgdhsrqchxvktrz
12-16 r: kglbvbwtpfprrshrwc
5-10 x: pktbxxxcxx
4-6 d: dddmdsd
4-7 x: xxxrjxcxxx
1-7 m: tmnlwczfmqmmhfmc
1-5 x: xxxhqkx
5-15 t: trlltgtdqbttwst
2-4 g: gggg
3-6 n: qnnmnnnqwnjn
5-14 g: gdwwwmgjrztgqcgmg
2-4 s: psks
6-7 h: qhbhhrhhh
8-12 p: gdhhpndpkxsl
5-6 p: pnptpp
2-8 n: gnnznvnn
14-19 z: zzkzzzzzzzzzzzzzzzzz
5-10 h: wrzdwvvwftdgnthkfn
2-4 t: lbnt
8-12 b: bbbbbbnbbrbbbb
2-3 x: xldp
4-10 s: srsqqbrjndfwtnvwjww
9-11 s: sssssssssbs
5-9 q: qqqqvqqqvq
2-4 b: jbrbtbtqfbqltb
8-10 w: wwwwwwwwwww
2-3 m: mmsm
7-9 d: dddddkdddddddd
5-6 c: cdxcccclc
2-5 m: bhxmrxf
7-14 w: wwwwdwrwwwwwqt
16-17 d: dddddddddddddddqpd
13-14 f: fffffffffffffd
8-10 b: bnfqbqbxbxbb
1-4 c: lcmb
1-5 m: lmmnbkm
10-18 v: vvvvvvvvvvvvvvvvvvvv
2-3 f: wffrjzmbf
5-11 t: tltkttgtqcttlt
1-3 f: ffxffsscddh
5-6 q: qdtmqqh
2-6 d: jtddnpdmdk
12-13 z: ztfpczcrzzqzzt
11-17 x: xfxxxxhxxxkxxxxxt
7-8 l: fhpcltllq
1-3 k: bkdkk
2-3 k: vkks
6-8 c: pzvhwdlkpfc
10-11 z: zzfzfzzzzzzzz
3-4 p: ppllpvk
12-13 b: bbbbbbbbbbblbb
9-12 x: nlxxcxtxhjhx
2-3 q: zqqqg
5-13 m: mmmmkmmmmmmmm
3-5 r: rsrtt
1-8 d: dkvnxddd
15-17 q: qqghqqqlsqdxqqqqq
5-6 l: bllzxlll
15-16 l: lllllllllllllwqll
2-7 b: brmbptt
2-19 m: wbjmmzmdmccnmjmmmxxm
2-11 m: gmmmvcfmxhn
13-15 v: vvbvxvvdldvnvtv
15-16 t: tkttvttttttttsbf
12-16 k: jkkkkkkkqkkkkfkkk
12-15 d: hdpgdjbktdbsdtpjq
2-10 v: vbvhvvvvvvwrsvvvv
2-6 v: vzvvvvvvvvv
8-11 q: qqqqkqqnqqv
3-5 h: dcrhp
6-7 b: bbpbbbrb
2-17 q: qqqqqqqqqqqqqqqqqq
11-12 h: hhhvhgzhhhxh
12-14 p: pppppppppppppnp
3-4 r: rlrrb
4-9 s: zsskkscxs
1-9 k: kkwkkkkgk
7-10 q: dqnqqqpkqqqqsgbl
11-13 p: ppppphqpppppp
4-5 z: zzzwz
3-7 j: mxjcjjjjbdfsjqhkff
10-14 b: nqbkbplzwqhzhdhfbj
5-6 t: ttnjqtqttt
8-10 g: bkkggbzgctgk
6-8 c: cccccsbf
1-2 w: twwb
3-8 n: snnqvqsfzhm
5-6 z: mvzzzqzzvz
14-18 b: bbbbsbbbbbbbbpbbbw
11-12 f: fffffffffffv
15-20 n: hnnnccnkftnzmszkllsg
9-10 h: gcjhhhhsfmhhc
11-18 t: ttttttttttrttttttpr
6-16 x: xxxxxxxxxxxxxxxxx
7-11 m: lcqggxfsbtphbqc
7-20 t: khkhptldltdrtfcttwcp
1-4 m: dmnxmmm
15-16 n: qbxxtldfstdstbgtrx
2-4 p: hhps
8-18 g: gggggggjggggggggggg
11-14 c: qccccccccclccnc
5-6 s: fsnscs
9-13 w: wkwkwwwwmvjrwdwwwvw
16-17 r: rrrrrrrrrrrrrrrrz
15-17 q: qqqdqqqqqqqrqqqqjqq
6-11 d: dddvdmddndbddtt
5-18 q: dvxkqjbqkkqqmtwjtmq
4-6 k: tkgkkk
2-5 m: mgmmm
3-4 k: kkkkkk
1-4 b: qbbhjjgvlbsrrtbbgx
1-6 n: lnnnft
12-15 d: hdfltndlzbcdzfd
3-8 l: sdlzgldl
11-12 w: wwwpgrwzwwpwwrgdwzw
8-9 f: dplhfcvffffrqf
4-6 w: ztvztwwgwq
3-8 c: zdpcccvzntfc
2-4 w: cgfcrslfwldbzsdxd
7-12 h: hhhhhhxhhhhhhhh
3-5 n: nnnbnnnnnnnnnnnnnnn
14-15 j: jjjdjjjjjcjjjhtjjjj
3-4 j: jszj
2-5 v: vvvvv
1-3 k: tkpk
6-7 l: lllllll
3-4 m: sxcb
3-8 s: wssssssctsk
13-15 t: tltrthphnvrfhtt
2-14 k: krkkkkkkkkkkkpkknkkk
4-7 x: xdzxxlx
11-16 j: jrrtjtjstrjtfjjjj
4-7 r: drrdbnjhb
4-10 q: thqqhgqqfrr
11-12 r: rrpwrrzrsrrr
10-17 g: gmwpvcbxtgjlcgznlrjp
13-17 b: knbpcbzxvnvnnxwhwnzj
2-4 c: ccctcc
7-10 t: kstttdlttn
6-7 b: bbbfbbb
13-14 c: txhkmvxwqjdjph
9-10 n: pknnpbstnnwztsjkfc
4-5 k: kqtqk
13-15 p: kndptjczrnqtphl
1-3 x: pxxxx
11-12 r: lrcgqtrmjjrrxhg
1-4 f: ffdf
7-18 d: cdddddsddddddddddd
4-10 r: rrrrrwrrrrrf
11-12 p: dpvmgjrmfspp
1-2 q: snqqqgq
11-12 x: fxxxhxxxlxbxxqx
3-4 p: kscf
13-16 n: nnnnnnnnnnnnnnnnn
3-4 h: phpqz
1-2 c: ccccc
7-15 f: fffbffhfkfjfftfgf
2-3 q: wqwqqspqq
12-14 j: jqjjjgsjjjjgjj
2-13 b: bbbjbblvgbbbbbbbdb
4-7 t: thxhnps
6-9 n: nvqjgjmxlb
2-5 k: kkmknkkk
9-13 z: szqlkssjlzzzbtcv
9-10 w: hwwwwwwwww
15-16 s: sssssssssssssshk
1-6 q: qqtwqqqw
1-7 s: sssssssss
1-5 n: nnnnkzn
9-16 x: rdxxzkvhnxwxlxnl
16-18 g: gggsgggggggggggbgrgg
10-15 s: sssszsssswcsssg
7-11 n: ndlwfrtcnntdjbtgz
1-3 c: cccdzb
7-9 n: bczssstrkctjxttmhgcg
2-3 d: dthdd
6-10 l: llllllllll
4-5 t: tptttgwtttd
5-6 l: llllzml
4-5 g: ggggggxglgdggggw
2-9 r: rrxrrrrrmrp
19-20 g: gpgbgkqggglqghgghtpl
5-6 k: skfmkk
5-13 f: pwwkfglvfntqntptggfb
7-8 w: wwwwwwwww
4-6 w: wwwwbwdsh
1-2 l: lllp
6-7 q: qqqqqwx
10-11 m: btmwwldfzhw
12-13 j: jfjjjjjjjjjjjjj
2-6 n: cmfsjnrhlsf
4-5 q: qqqlbq
13-15 z: zzzzzzzzzzzzlzzz
4-6 m: mpsqmhmzmn
5-9 s: dswmrsxvkwrfnc
8-10 v: vvvvvvvvvvvv
14-16 h: hhhhhhhhhbhhhhhhh
9-10 p: bphppnwzmpp
6-7 x: xxxxgdzx
3-4 j: smshf
3-4 h: whnj
6-8 z: zzzzzlztzz
5-6 v: vvrcgvlbvczfbqvv
1-3 v: vvdvv
3-5 t: smtgts
6-7 b: fbbkblbxpbnrw
12-15 t: ttttttttttttwtvtttt
3-4 s: psss
13-17 x: xxxxxrzxxxxxxjgbxxx
9-10 r: rdmrrhrrrrprrsrrh
4-5 j: tjpgtjn
2-11 n: ncdwnnwnntnvpnnznjnn
3-5 d: ddmxtddjddddddd
1-5 z: zzzskzc
1-8 d: dwnlmtjdfsdmddrsdwd
4-11 k: hllktmpbndlgphxb
3-6 n: tnncnnnqdbgmgnn
12-13 x: xxcxxxxxxxxctxx
5-10 s: sssslsxmssss
1-6 x: xxxxxxxv
6-9 n: nxjfsbqppwhnn
5-7 j: jjjjjjjjjfjmtln
6-11 d: dddddfddddddjd
3-5 s: fshssvps
1-5 k: tkkfhkk
3-7 h: pwjbmswg
1-5 k: kkkkkkvtkt
8-10 j: jjjjjwjjqj
16-17 g: pggznvrnxlgdrlvgg
2-4 z: zzjf
2-5 h: dmltcxthx
1-3 m: mmmmm
14-15 g: ggggggggggglglgm
4-6 m: wvmmdm
11-14 c: ccccccccccccdcz
9-10 w: qswzbfwbww
15-20 s: qblspjsxzpsqlgktwssc
11-12 q: qzqcqkqqfqqgqqqbpww
3-5 d: ddfdwv
2-10 t: vwjlkzjwztrmrmbwftnj
8-10 s: sssssssssx
13-14 r: rrrrnfrhmrrrrr
6-10 l: mgpjzjshjdc
1-6 l: llllhlldll
11-12 q: qqqqqqqqqqqq
3-4 w: wzzsk
7-9 x: xxxxwbchx
3-4 k: kkbx
8-9 h: bhvhhqhhn
10-11 k: bkbkztrzrfk
3-8 p: gdptpvpp
11-12 s: ssmrsssstsbj
3-4 w: lgxvgbkwdggwfhwn
8-12 z: xmdzgsrzmnck
2-3 g: gjfvtgrrfg
9-11 n: nnnnnnnnnnnn
10-17 d: dddddddddqddddddgd
12-13 b: bbbbbbbbbbvxb
10-14 q: qqqgsxdqdpqrqqcb
2-6 n: fhzqdzp
3-5 n: fknfn
6-7 g: gvtqlgz
16-19 z: jzlhzxtvgzzvfvhmrsb
9-10 z: zkztlzjzmrr
14-15 p: pppppppppppppfpp
8-14 z: zzzzzzzszzzzzxz
11-17 f: vfbzfcffffffffffpf
16-20 p: hqgnrpzcxvqhgnqrbfcp
14-15 x: xdptxmxxnchxxklcxzrx
10-14 p: wxtxgvncxpgppppbflkp
6-7 k: fllvzqjkpkkjnlnkbjk
2-4 p: hxjzvkqflmktcvpk
4-8 m: zpbmcqhmxnbmpmdhkdxz
5-7 d: bnbdcccnvcdrpg
15-19 g: gjgggggggggggfggggj
7-11 v: kvvzjvqfjvlvksrphv
2-15 j: jxzwjwtjjxlxjjjjjjjj
9-13 n: nnnnnnnnznnnznnpbn
8-9 j: ljjjnjgjjvjdjfjl
6-7 q: qqqvqkqqn
5-6 w: twldlwww
10-13 k: kkkkfkkkkglkv
11-19 c: vzgccvbksxwcvhxglkw
3-5 h: hhjhwp
4-5 x: zxkcx
2-3 m: rmjrvlsm
2-4 v: vvvvvv
1-5 g: ggggbg
4-5 q: qqqlkqqq
4-5 j: tjznjjm
1-12 g: ggggggggggggg
1-10 s: dssssssssssss
6-7 r: rfrrrrmrr
3-6 s: hljhqs
2-11 j: jfjjljsjdcr
7-8 m: smzmmmmm
6-8 h: mhhmsxhqthcfb
2-9 w: jwlbwngdw
4-7 v: vvvlvvzvv
5-9 z: bgzbzpfvjqvkbqstg
6-10 v: pvslvlvlvv
6-13 x: tbmsxxxjdwnfr
11-13 r: rrzrpwprrrrrrd
14-15 k: pdbnmhrfppnxfkkvfxvk
10-12 t: tttttttttttgtttt
6-8 g: ltbggnrgr
9-12 d: dpdddddddddd
10-11 q: sqrqqqqqqqq
4-14 j: jjjjjjjjjjjjjjjjjj
15-16 c: cccccccccccccctc
4-9 z: gzzzzzhzzbzzzdv
2-13 x: glxmkxxxxxxxxxx
3-12 n: ccnvfnxxkpdxnpdppcn
4-7 w: wnrwlzwrlbrrwvs
13-16 t: tttttmttttttwtttt
5-6 l: cllhqlzlblv
13-15 s: mtfbvkkjznwsssl
2-13 k: lksdvschkvfchrwkf
3-6 j: fdjnjbf
7-8 l: llrzbldc
2-4 l: ljnrv
1-5 h: qhhpt
2-4 k: rhwkqk
17-20 d: cqdxmvnqtdddddznnddl
1-4 z: zzzzz
3-10 v: vvgvvvvvvtvvvvh
4-9 q: fwqnjqqjkjq
9-17 r: rrrrrrrrrjrrrcwrtf
4-5 k: kkkzkkkk
9-14 m: mmhmmfhhmmmsmm
6-9 h: hshhhhhhh
3-4 n: ngnnn
8-17 b: zbvbpgqbsrzwmxzdbfw
4-8 k: wktkzcbg
8-9 w: wwddwcrwwpkbswswwkx
5-13 d: dndddpfdzfdvpzdhddd
11-16 p: fplppmnxvzpqsngpp
1-12 h: hhbhhhhhqghhhhhhxhh
11-12 z: zzzzzzzzzzhz
3-5 h: bhdhkl
3-4 h: hcvl
3-4 d: zbddddcrpgdxddt
17-18 j: qzwfjjkfpkggsjqvjwx
9-13 j: wfjtjjjgjjmtjjwjlh
2-5 f: xdpsfkpzvfmhf
4-5 c: cccccc
13-14 h: hhhjhhhhhhhmshhh
9-14 t: tttttttttttttntth
5-6 s: ssssdks
1-8 t: tgbtdkht
4-8 h: mhbhpvhh
17-18 x: xxxnxqjgxclqkxxxxx
2-5 g: jmvmgnghr
13-16 n: nnnnnnhqnnnnnvnnn
3-7 v: ngqvtvfbdlr
8-14 c: cnmrshvhlqnchtcbrgh
7-9 n: tfqswnnrg
11-16 x: kgxgvcftmxgxbnxg
2-4 n: zqnd
1-3 w: wwwncwwwkwfww
8-9 z: zzzbzzzzzzzzzzzzz
17-20 k: kkkkkkkkkkkkkkkkkkkk
3-5 q: hjqrqsq
12-13 h: hhhhhhhhfhhhh
3-4 x: gxxjphxx
4-6 g: sbwggg
";
