/*!
# Day 6: Custom Customs

As your flight approaches the regional airport where you'll switch to a much
larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you
need to do is identify the questions for which anyone in your group answers
"yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language
barrier and asks if you can help. For each of the people in their group, you
write down the questions for which they answer "yes", one per line. For
example:

    abcx
    abcy
    abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c,
x, y, and z. (Duplicate answers to the same question don't count extra; each
question counts at most once.)

Another group asks for your help, then another, and eventually you've collected
answers from every group on the plane (your puzzle input). Each group's answers
are separated by a blank line, and within each group, each person's answers are
on a single line. For example:

    abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b

This list represents answers from five groups:

- The first group contains one person who answered "yes" to 3 questions:
  a, b, and c.
- The second group contains three people; combined, they answered "yes" to
  3 questions: a, b, and c.
- The third group contains two people; combined, they answered "yes" to
  3 questions: a, b, and c.
- The fourth group contains four people; combined, they answered "yes" to only
  1 question, a.
- The last group contains one person who answered "yes" to only 1 question, b.

In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes".
What is the sum of those counts?

## Part Two

As you finish the last group's customs declaration, you notice that you misread
one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you
need to identify the questions to which everyone answered "yes"!

Using the same example as above:

    abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b

This list represents answers from five groups:

- In the first group, everyone (all 1 person) answered "yes" to 3 questions:
  a, b, and c.
- In the second group, there is no question to which everyone answered "yes".
- In the third group, everyone answered yes to only 1 question, a. Since some
  people did not answer "yes" to b or c, they don't count.
- In the fourth group, everyone answered yes to only 1 question, a.
- In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes".
What is the sum of those counts?
*/

use aoc20;

use std::collections::HashMap;

fn main() {
    let mut input = aoc20::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 6: {}", PUZZLE);

    let mut maps: Vec<(usize, HashMap<char, usize>)> = Vec::new();
    maps.push((0, HashMap::new()));
    let mut idx = 0;
    for line in input.to_str().lines() {
        if line.is_empty() {
            maps.push((0, HashMap::new()));
            idx += 1;
            continue;
        }

        maps[idx].0 += 1; // update group size
        for c in line.chars() {
            let count = maps[idx].1.entry(c).or_insert(0);
            *count += 1;
        }
    }

    println!(
        ":: Sum of questions answered yes in groups: {}",
        maps.iter()
            .map(|(_, questions)| questions.len())
            .sum::<usize>()
    );

    println!(
        ":: Sum of questions all answered yes to in groups: {}",
        maps.iter()
            .map(|(group_size, questions)| questions
                .iter()
                .map(|(_, v)| if v == group_size { 1 } else { 0 })
                .sum::<usize>())
            .sum::<usize>()
    );
}

const PUZZLE: &'static str = "Custom Customs";
const INPUT: &'static str = r"
we
euw
we

czaxvodqbsjeytwhurpg
gclajqykpmxfbohvtedzwrus

dxoznjqhwuvblprgekyfcm
ghbozdkxqjevwfrypcnul
rxpjtgwvuoeqfhlkncdbyz

ralobiy
lyairo
rzyailop
falyismnr

rvwzm
rm
rm
rm
mr

purjnligxyzmfbh
ulfxmivrhnbq
mfhrnxbwuil
bdmrahucxqlfins
nmkeafixhbulr

vgqjuk
tjlou
upljm

kyevcgxzljtu
czkevygjoxltu
ujxegyltvckz
cyeztvkulxgj

nj
jn
nj
jn

ewdrahozqptylubxskgcv
qenzcgprkhoxltadsubyw
uoskeaqrgdplxtzbyhwc
xpyuartbowhcegkzsqld
ywpozkblrcuageshtdxq

jzkqs
twelgfpyn
juzrbxdso

ocqflkxdwbu
lpvsgniazehtjr

khfijnszyrgupqaomx
csehqtuzbgnojkrxmv

m
m
sh
m

yxbigdas
psxrmajzilbcftk
sxbyinuvoa
usegdbxiaw

ezlfkwt
eklft
etvbhluafxscnp

vswdrciajbnufxpkm
uridvfbxkmnacsjwp
asvfkrpxcbmdniwju
mfjbdauspwkxrivnc
cjiwmknsdpvbxafru

sqidvpzjbfxlntyomhkarcu
vznqykmxaupfbtdrcoh

qstyakgcifh
sfkqictg
vlcsizqgtbmkf
sbckrugvtfiq

vgbmj
bamjvgi
mszcybg
igmb

wdeych
yhdwec
yhcwde
hdcwey

ptzmolyxkbjaecgshfid
jzycdaflqbpesviotmgw

wikvsjy
jwvfiy
vwjiy
yjiwfv
vjyiw

smgd
xgdsm
gmds
gmsd

tgxhilencfaduz
mulrcfpeqidtgnxa

kjgy
mautbnlds
piezhc
cyx

esxuzqa
exupzaqs
sdxqatlzeu
ufxspazeq
xwuszqea

fak
fk
fk
fk
fk

xvtzoyiq
sziqyovx
yloiqvzx
oziqyxsv

wb
bvz
zby

dbnmwchzqavgxs
plawsmdqgfthixn
goqcsxdhmeajwn

enogjl
icmjxdbwoa
vohwuixj
jvo
fokqjt

sedzmian

mpfijqk
dnwlotuv
fcbaj

c
omp
yk
w

u
fq

qkvm
qvlmk
vmq
bvmrq
ifmqxagvu

tkmapfqrwzxn
zmrpwnkxtqfa
ftqkampxnrzw
waqfrpxtnkmz

znmlgefvodhywbpx
vslwxbhupjqa

efo
eaw

lf
mqux
k
y
k

jxcynmtkhlue
rkxhtuyjlz
uyezktxrb
fysxqktaovw

fvsyztlrbmukhnoagxd
mpcauqvzgilsorxdnb
oadlgnzsurxmfvb
sbdoelzmvnrgaxu
snxtdramgvuzbokl

znc
uazi
odamuz
zju

n
n
n

msuxdewqgoya
jsyoaup
sjazuylo
zapousty
auofsyzt

rak
syzlc

wtvyqfumsodjplraezkg
mjzktywdcvluaerpqfos
jzrovtwlpqusmeayfdk

fp
ws
opf

xjp
zbgj
ljzf
fbj

w
w
wx
xw
cw

lwxbqfcmvuyoi
fi
fgpnti

lpxmcsq
mpcqht

fuyx
kheslrvmo
ax

fbx
bcx
cbx
xb

hncwogykebflqpruazjmidx
thgpiucndxlawfyoekjbmzrq

sodcjlveytb
eldjictpbv

edafplnw
fw
wf
fwx

dgvqliaxyfrkbosmucn
calndmksbqiyhrox
mkabsonxdqlryci
mrlxyikancsbdoq

ieyldok
ykldoei

vyah
hxyev
cyrqhvfd

fexskjpcgzvltyrou
pxviozhjylwetnc

wpeimqgotbrdjy
omrwgqdpjti
wojgrdqhtpim
jpgqtiwhmodr

nrutmvycdpalhjixeqfso
ojvhtrxpgcdeukasm
amozrvdjtsxhguepc

rfyqnmoix
rxo
olxr

upjcoawvbiqtgerx
zdufmyoaxksjhlg

cedjq
sdecvgxqjno
idecqj
cilkmdqje
dqerjcm

tgfnq
m
pziohc
pl

xv
vxy

mqatcklidyjxzoewb
iljoazdtwqxecmykb
wlqtxyzjmckeouaib

kiuhoxpqaefgwdjblm
jdbeqmhaipfwuxogls
hzybqgiwomxfueplajd
fodibgexuwjpmlqha
mfobdkwihxpjasqelgu

oubae
uebaxo

ozspjekdlxcvgwn
zjonpldkfgeswcx
qdbjkwnpyhezxcusog

uwptn
xbhtwdkuv
wtnu
fwjutz

iwsvtckjulezdnxabo
elspdwmvxianjctuzbo
qtikalzvydujbncxweogs

valcwrzboskdpxeq
laknzihxgfdqrbpov

wtj
jwton
jywt

qnbkfilghao
xoihp
wioph
hiocdszx

dx
dbg
idopfrqk
wdyhemtjc
kloifd

tvuey
taemv
dgeophtq
wsvtane

htmqiurgfzewbnspad
pejhgqvnmdzbfwrkixasu
pheiqalzdyugscrbnmowf

nyub
yrgunqbvi

flt
ftlr
ftl
ftl

svkchrbp
mcx
krgc

vmpanewckdblu
ofxrhqytsgzjvi

gio
icoq
jtoi

j
w
e
w

yfhgcwzk
krcfhetgy
yerunchgfsk

mofzbljpiyct
bymcjp
mcjybp
juycmpb
bcxpymj

kaqri
iaqkr

tdsfr
rsad
drws
rsd
rusmd

fhctzixskvldmnrupgqbwye
bnzmxphdskvwlcyetuqrigf
gakcxpnhezurybsmfltqvwid
mkrvqcfgunhdpbowzxyitels

sgoha
varskijcp
somha
shal

w
c
opz
w
c

nwjor
henlwj
onjw

gfbexckpyonsjlrtqd
godblnpqerjckyxsf
bheojgdrlqscnpkfyux

owihczayl
byhldoz
xyhzvol

tlvydpageqcwm
qvyplwdt

ykdxtlsapfe
lbkdoarpex
kjmueptlaxd

fyw
rf

sgjqa
jasqg
jagsq
sjeqga

oyftwsjnr
jcyltkosfpdwn
fnjyomsrtw

lkugoqn
nlokgu
lognku
ngolku
nkoglu

jyq
i
l
q

i
m

fbmorsgalytzqcwujvih
dpjythovwnizslfqmcarbug

afmdey
edymf
mfyde
efmyd
dymef

ge
r

zdwlhekqyrvfm
ivjhfdwzk
djkzwhfv
hkvwdzf

aimdg
dagm
wmagd
dgmoua
mdag

pdgxvcsr
sohwekxmftdij
sqvnxgzd
xdsqpavbuyg

buytcfgqsvzrmkdhepaixow
zaghwdrtmyikocuqsexvpbf

vsbrnpgomyia
ojaglmpsr
gzqykpardwsmefov

h
h
h
hps
h

parjim
pjrma
majdxhpzwrt
gjmarlp

bapshndgv
hsxdbnvpg
schnbrpdkvga
psvnryabdgh
btoeisvqdhlnfgp

lmigad
zjwxpqhy

xwjzf
pjwxzf
lkfwxtnzjh

y
y
ty

njhwpxdoc
qdhxnpowjc
rhdjnxawcop
qpcnohxjdw
dpwcxlnjheo

lpfzgcmutnbrqy
mplfqtuzyrgdb
ztwbjsiguhxqykfropl
qtlgzubyrfp
nrtdbyufpvgzql

cshxbdmwfzk
jterbomiahwlf
fcmhvbxzw

dvubownmsyaqr
buqmydnaovwsr
vqwrnudaysomb
asvbqnryodwum
qybwausvorndm

tjnlgypkxrswd
yurlpdwxntjsg
lcwjypnxtdgrs

uwbegchfszrtpaq
frhtcwuesjqag
uqrhacnegfxtmwsk
fedpruwagtvqshc

h
h
h
h
h

hyp
hpy
yptiv
yeupkh

tpa
tpa

tyxb
f
fi

w
i
w

qlrn
rmnl
lrm
yclrzi
olhr

pzyojshleakmvx
mfhlpkwzvngsedycq
erkpvzlshimyb

iuwypcshemk
gvcuwylki
qocbayziwtnu

sqxn
xsqn
nxbqes
cnsqx

eskfqzcjgbndmx
jnbqckdmxfegsz
dusbmcfkqexgnjz
mfjkqecxdbgszn

haigdtzxjpvlye
ebdphavzlgjity
pyavulrzdigjeht
ejgihvdwysmztp

howzjupcbrydeiskf
pxyajrewscoundzihfk
pgwkhqsiolmfcdrjeyu
pedywvsaforchuktij

qyimsojl
mjqisoyl
jmsylioq
imycqjdsol
jisloymq

q
dq
z
i
nso

pfcvso
copvsf
fzospvc
cpofvs

ungm
nug
kypibtcqz

xdrs
drx
rxd
drx

egkmnsvuaqxpocdjziybtfhwrl
dmgolkspchqjaxfnvyubezirt
oqartdhfnmcusziygxlbepkjv

b
b
b
b

weugmsnk
vam
px
nrwk

njwlhfkzbxosiqmtdaep
tjdexnafhszrwoclmpbk

spucgrxbhnkoyd
xgnchbpuydsro

kuvbydenf
egmfkcjuy

rubywdkos
jcdkyuspb
ubksfndy
xilhybkvueqtsad

kszhqepbdtauwgfvo
sdhwugptar
tdgalsunhrwmp
uytahpcgswxd
jauhwpsgdt

norelaqwcdikp
idqxcnlprteo

guwztovasr
wrfgteold
girjqcnxtmw

vbesoyprzckwhja
ehcyvrjoawkbxspz
bjqewozpcavhmyrks

fuek
fmud
ufzwt
iqxtuf

izxsywjpnghk
zxijypksgwh
ipgwzksjyxh
gijkhzsywxp
wsikjgzyhpx

oj
oj
jf
j

lpzxcesvaywuq
egxqwzshpvuclfay
vqaucwspyezlx
plsmucwzaeqdyxv
lxaceuvsypwqmz

nfk
znefakp
nfk
fkn
kfn

vrwasxfehckpmyubnj
hnruaesmwfjcpxiqlky

mkpqb
kmbq

ipe
iep
epi
epi

tzpm
urjvs
wyfu

x
x

awihujr
waihu
hauwit
dhwuia
wuaih

ixmreldjczsp
msixaqpryjdzgc
cxiszmrpjd

xihrglkawevfcztys
wvfxgerzkslthcyia
tcykwezsxriglvahf
ltycdnisfgvxahjerzkw

jctmxhldyfekbsnoz
fsmjcoxnehkbdzyt

a
nzqihgm
ajo
xg
kbtd

qpjiecwsntkdfxoylrz
opxnjbiwrsdqfteyzckl

cskztdyhapb
hfdzytskmpcjv

dsgqceaxhjlnfuptkm
zupenxmchwisklgaf

bnfxiwmgsaylkruhpeqjc
pmxljerbnzcuokiqswh

abhiumenpwyxdtg
mydghinwauptxe
dtvnapmuiyxwgceh

hm
i
v
i

srqagnl
glasrq
tlsgerqhau
qgslaxr

camkndvibhglfsxjzpy
ipcnsmaybkdfuhzgjxl
iywsnxfzlqcdtjkmrahgbop

eowmyghk
yomwhgek
ymwgkohe
emohwgky

bt
bt
btzky
bt
pbt

o
o
o
o
o

ebuvrfzag
zpjusrbhvnq

oxwkzil
wozmuik

aermnpwkftbjygidcq
yhzvjurgcdnpsx

zmynefsaxuldrvkochbpqwji
ojndmyrbwevhpfkxcziulsaq
owcdaxgymnjslfkvbiteuqhzpr

mzybidswrc
ysimrczbdw
brdcmiwzys
scizybwmdr
dbyrcgsimwz

gjkhfwri
jkhifrwg
fhrijwgk

rjbyic
cfijyb
ciyjbr

helktqgyvmwxpcuiznorf
mlxkvurotfpneyighzqwc
kcfnquprwmzhieovxglyt
vypdbmqnrftugeoicwkzlhx
hnzcfiemytgrlxvwoukpq

tpw
mxwfelqr
pwid

jgncopdsbtkiuywh
ikoctbuhydwspnjg
cdpwhbnkjtoigysu
wducshinbgjyotkp
owtugkcpihdjbnsy

vcizx
pujwzqtgoafhdm

l
ml
l
l
l

n
h
p

ag
an
a

trgboyjfmscqukizxvld
kvrglmxcdjqtioyubsfz
cldvrzgufqtmjksyboxi
msgqiolcxbuvkrdetjyfz
rmxbfiyloujzgtsckqvd

cyg
zgyc
gyc

mvkxejgpqanzl
paqnvxmezjlgk
zjexvkaqgplmn
nmepakglxvzqj
emkqlnjgzvpax

jrdpfenlugox
ckfxvwtlrio

wcdxojeamgsvni
xcwundovajiemg
mvdjwoiqgcsenxa
dixacneojvmwg

dislpne
lbrofx
il

lishmxzbecv
hcvmbiezsxl
zlhbvmcexis
lsvbichezmx

chbrojd
dhjrocb
bhdcorj
dohcjbr

pizahyjd
djpayhi
ecpydjahi
iyhzdpaj
ydphaji

sterfnwkbmy
nkvlyjtprfdmbsq
ntmrbksyfc
ncbrmktsyxf
nfekrsmtyb

vdoaguf
aogduvf
dogfuva
hgravdfou

hmy
hytm
hym
mhy

ebl
ebl
elb
vyblep
bel

kaiftmzwsbrqcx
tzinrcfaoexswkqmb
pvjsfcqzmltxkgwbi

jmfbunxys
wplzbevo
wicbdeh

zuaxbhksnjgcpdmwvoiltf
dspmizbnxqafolkgchuj
escpnbazluodxkihjgfm
nbmolcpdijgxzfkshau
uicnzjgkampfsldhoxb

obqewjzkpuxfhmal
zxlmkbaqtpweuhfoj
oafmkbujqpewlzhx
xpazwjoeklchqufmb
oxmdzbnufljsiwkqahepr

jludixebmrghfy
auhergixfmybvlkdj
lmpubnscixegjydho
wyxmeudbihvljzg
ijydlxgbqhmuzre

lfq
kjlq
sdhtlqnp
zql

evwzuc
cvzweu

aqf
qfa
qfa
faq
aqf

dzaoinjrulqectpmbkvh
vonhyijrzcbmuekdtql

kmir
rkmi
uimkr
mfrki
mirk

aitmegusokn
syvgitzmoluk
wjkiurosmtg

b
w

apqhilcg
sgpilcaqh
cipqhaugol
hligcqap

sqjtpov
uzmhct

nojxhldregqytbu
gbqxjodletnryh

xuk
xukh
gqukd
ku
zauk

nylwmd
csermkpjvwn
flotmwna
oanwm
zmuqnwl

qeyofrigandz
cpvsimnzlbogju

ckuwoafg
wragco
cgmwqao
rasgwxcto
ahwogpc

d
d
d
d
d

tscd
vmicosrfxed
tcds
ucdlsj
jhsdcb

ozwbenithjqfp
tjeiknowpflhbq
eotfhqinjwbp
oqihepbwtnfj

owrczyldnba
bczwroudy
ckywdbrozi
zuwdyorcb

kcz
kcj

kpz
zkmp
kpz
zpk
pzk

vuahzj
akquv
vawzr
bpafgesnodv

wuov
wuov
uovw
uwov

tplidfauy
placuzyfit
yulpitaxf

ibszxovdprhquamye
oahqufevzrimdbxsy
vdsmzhboxyraiqeu

wmcpthzklyr
udb
degoaun
jvaxibf

tedjwhunalzvirqoyspmcfx
jenufahxqpcdrtvwmsyzl
scnzwpfxdyjtlqraevumh

zf
pfwgi

i
i
i
x

jonzkcad
zdjkanc
jakdzcn
njdzkca
kacdjnz

olfcubmsy
flmysobcu
gyofmsuncbl
osbfucmyl
bmoylscuf

mfezpgicq
scigqzopmf
gpficqz
ogfpilzqc
zgfpcwyqi

kxdtycvposhlj
etqjmygkbwhcoridalusf

gbjqdtaksferwnpzcohivl
qujvkwrfnpbsatgcdleoihz
aisjdexzbrcknqftghwoplv

e
y

uo
uo

dypktis
onwlvfy
omy

rqxewzljfomt
jomhrzndweykuqt

k
kvec
kb
k
qzkugad

tnwvjfbr
wjfvtrbn
wbrjntvf
jrnfvwbt

ohxbfvg
govbf
gvfwob

jq
qj
jq
jaceqfx
jq

wuigsxrhfbvpecykodtjzmln
fcrmneoyidtxlzukshgajbvw

zh
zh
hz
zh

qxwt
xtq

lu
xl
cl
qtli
xl

jegyxzqc
xulzcg
gdczx
gzcx

hvbajsokwnu
gahubjnpotv
jevtnaoubh
ajyhnofbvu
dunjioazmbxvhlr

tgodl
qz
sq
c
izrq

dy
vdm
jfvd
dnq

vdjunwyelmbs
msvnkdblwueyj

pqnyegvuzhawkof
sniegrmuwcqphjvfb
qfnhvlwdtpgeux

cutedf
udtfce
ptvdfecu
fedutc
dfcute

xzvjf
cwiej
mj
vjk
gjlb

nwb
bnw
obnpwe
nbw
wnb

j
j

hcqaeovgrkysituxf
gutzhkxlrcbdoeqiysv
kuqsyotrvcxeigh

sagmjztfri
lmysfjbzgua
sagjfczmx
dwfzjakopsgm

rdxvpkenmgbcujhysf
yjbxdurgemfkvcpns
bsuknpgexfycjdmvr
xdevcrplnzuayfksiomgwjtqb
cvnehgyjufpsmkdrxb

ghn
ps
uorkdxyab
hmigj
es

tkdqsxyzij
qyshjaixtdzk
qukytjzdgsxi
xtziqmykjds
ytzisxdjkq

ykwnetlxqhjmozca
xmjoyahltwqznkec
yhqknjcelwxotmaz
ytqchzmkjwaxonel

rhjblduiefc
ilevpurjh
lvuikhjer
igawtheyprluj
qnusrhlexzoji

ojm
jf
jf
jf

cfuejldrabhqoi
cofwhtlkviaqurdpenb
iufxloqbhezcard
fchaqulymeoigrbd

webjty
tyebwvp
tybwej

p
p
lip
p
p

zag
gza
zga

cy
yc
cgy
xyc
yc

nl
nl
wlxc
l

doqnyeaxrtkbz
rqyzxtd

nuzcfybklvaqx
davcyslnm
srnlvcay
lnvcjaydm

zarquswjlbohemfpkcx
xpoclwsemkqbjzfauhr
jmqcflkwboprzsuxhae
lujxmfraeckzpoqbhws

vdzkwmrqycjbpgasu
hajtqznydpk

gwofesvi
iwoevsg
gyaevoiw
ioxwegsv
viofesjgw

srtgyzfad
wrbjmqfx

plstmbvnurofkjxicwhey
qpifyvmhlwacdrnojtu
rjcxpozifumbhyvtwnl
folmcryeunivtjpwh

qreavkshtgbcjuzyxloipnmf
mtrzbsijoekqhnfgvpxylcu
byurvjkicqtszpgnoxlhfme
tvjcqxursynglezhpofmkib

zkyiuobadeptvsnrwx
fkxypowlncrmziaedst
tzygwreshkxianopd

yxrnapgotzkhb
hrpdmfvgjlwseutqc

conmfxpvbaezgdukyr
ldabtmyoriecpukgxzwf
ykbdxoarguzempcf
embdczgoxupfryak

coxkulveqmn
mvjcxokulqpenwi
uxomlvkqnbect

dnpvgertym
btyihjmzaukrg
mrytclgn
rcypmgt
fmxpcvgrqyt

jhvwdcyxkifpr
pjqwxkdhvriy
hpwvrxdjkqygi
dhxikvrogwpjy
oxwyhdjvikrp

yehkjxvcts
ncvyujtx
ijctxymv
cqytxvj
jvctyxp

gskitvbwlryqjn
wjitsgnlcqrbykv
tsiyqkvogrlwnjb

ykfvmci
ckfiy

ypdsmj
jdpym
ydjpsm
ypjmkd
ymdjp

kcq
rjdsmby
tc
ac
e

misn
mpibln
vjmqiun
vginrmw
mnrciv

grluktqvb
lgqe
oqlg
egyqadl

x
x
fx
xf
yx

bqawrfgenpmsxhi
ecmfbhnwqixp
lokujvdqeyfzpxht

ngsmytarlzkbjp
awlingtpyrjz
drlgzptjany
razjnpqvgtyl
fjzlapmrguqtyon

ypefwhmuxztlas
xetwqpauvizfnrly

xrpfy
piyorf
yprf

rnuavlkitmb
iltuvaknmrb

whlunqaickds
hasueqjlbdgmkni
rzluidhxvqfpaoknt

kropubfxgvnqswjyhtdaiz
pwughlxbksrfojyznavtd
kfpwtrbnjugadvxhzsyo
tdpxhfojuvsrygzabnkw
vkyhnsdxjzoruptbgafw

ksmpflohdbagyuzjictnevwq
oebxmzngkwdjysqtipchfavlu
cqsanfdupebyzlhojvmtikgw
oeckzbigqymfvwljadntphsu

dv
lv

melukvncfqstabdjozxgwphiyr
gwkhnptzreualjmdcsvxoyfibq

twlguehvdazmfqiojk
gojvutzwdefmklhiqa
ltqagzfnujhpmvkwdoie

hgbcpaor
pcarbhog
bhprdayvgocf

li
wci
ik

ovubtpmnxwhkq
xtwkmuqhonvpb
bthqmxnkvwpou
muonkxvbhfpqwt
nthkxpovmwuqb

lmnvtcowzkqgfbyuisja
usaiehmxjcyfgwpnokvlrzqt

hbxq
bhxq
bxhq
qbuhx

zrifkodmq
frevcmpnolyxtk

shngpfat
tgfpsanj
anfspt
anetsprqf

tqyewczfjgpkardb
waqjeutcygkpfrz
ejqkrnywtfzapgc

eqowgydcx
qcwgeydxo
eycgqowxd
epcydqxgwo
gexdycqow

ztpi
ztpi
bzipt

zqdagnlikybcwmp
vsuertjofhx

jztshvpn
pnhjzsvt
nshzvtjp

bveys
ehjksvboy
yisbupfgmr

ezhgrayxkcjoifvlm
gsozrtcjelfypak
zkbwyctgenfrloaj

chispgoyvkwune
nepyswovihkgcu

yqhvn
yqzvhm
hsyqxw

fdjwbuqr
aqviephkwgyotr
rnwbmqx

htx
c
j

gwfemyktb
jemht
jterm
tqjme
retdhmc

qf
fq
quf

onhyrdlsituwazegfxkv
uztoevrknydflxwasig
vurfigxksntozaywlde
yxzotsfwakerlivgdnmu
slryuezxfdinvagwtok

bmfqglhuytwdkxvsr
osyuwvdrkbxhltgq
dtghxswlyjkurbvq
ybluqsgxrvtwhdk
lrqvuysdkgtxwhb

rywoqadxnskeg
nwreygskjdxq
gqyrsdkvxawen
rdqknoxbwyesg
xqkedswaonrgy

qjutsykgamlrdbxwi
riswxqjaudltmkby
jykudtirmalqwxsb
wrxjtamklsubidyq
uslabtxjqiyrmdwk

drgaqbehipluynvj
bgwhunrvljyqpi
jhybqpvgnilur

bzaxfsnrgvp
fpaniojbvs
nbavqspf

xghdtenswqozrfapkiy
fpyaktnidzehcxgqrmjwos
kdysiwnxzqfrhtgapeo
eprdlkifoygwaxqztsnh

qmjwx
wtxnmj
jnmxw

vwdfnjurhxqm
fejdmnviqlchrb
xqfjtrhmdkvn
ponasqgvmdjfhrz

jhrwkqblgotvfpzesi
kpiquwhjflzgrsbove

donwet
onew
zywenko
dowen

rfck
pmnqwr
eothysdrbz
rag
rm

lyqkpat
aqktlp
klwftaqp
aptqkl

lketoypncrqamwdu
ompehnkuldyr
kpyodulhenmr
yuprkmonedl
ihozyprlemknud

xwmhyk
pwsxhbnevgcizk
wuaydxrhk
txhqkw
jokhdltwfx

aiwq
wkqmaig

pormh
phro

dktomgpvwlq
powbmktgq
gliwtovmkfpseq
hpgunokjqmcxyzwra
pgvkmsowqt

vasgtwrdnzkjlimuoq
vczglrndtsqojwipu

umesvfw
whsum
zsmuw
zumsw

isogy
clsrq
xufjtvnzebmh
gkwaoql

hepbnwxlgiru
uemdhnogi
unhegi

exulp
xybep
wqexlp
xcpeflq
jgxehp

ouqz
zoua
uoz

omfybx
wtpkhgrdjlecsv
bmzuqanix

q
yfr
x

hkq
pqhn
hq
bzrhojq

eakwxyubldsh
auskeylhwdx

pbz
zb
zpb
zjb

ewfhgz
hxlqnorbw
uwdahmk

pzrscjd
pszrdcn
rzpmdcs

cfoemr
ocfmeir
foercm
recofm

s
as
vs
s

auntpvyk
nyvtpk
vtnkhpy
ktqvjwcpny
nvykthp

jfaclveirwht
cerlwatjuif
zrymljiwocfap

ldbkzysiwfuqgecxhjov
hziyqxfwubjkodglvse
zjyiklhebqvtwodxgus
jgwquhxvkdiyblozes

tlrg
eltr
trlk

tdiq
lxhwya
eusfogn
cypdia
tdkc

unbs
nqbu

s
s
s
q

ld
zld

fnwjhumltpkav
jusvqfbawpk
fpjtkuwiav

obuislw
owi

ukrospcnlqmva
ymrvupknoafcj
bczpvknmroua
rvakcqujpomn
wkyomaqclnpuvfxr

wvitsxphb
vstxid
xhvitws
ilrxpvtsw
qutsxvcikajg

uahlxcfkqymb
zlmashtgpybu
fmhbludavyc

yw
ywq
wy

qbkzsmrc
srtkczmbqf
zrqmcbks
qzkrmbsc
krmszqbc

gb
oibg

q
y
y
y
y

vdq
mdvq

bpe
pbe
ebp
pbe

h
h
vo
h

sadituvyrxq
anqhstrbkuxvo
rsuwxcamqgtzfve
htrqspuxva

yhogpjtzensad
wygoskparejtmznd
psdhunztajgeoy

fzgrtlsxy
yvgftwslx

ojxdegq
jqdegxo
doqjexg
qgeakdoxj

r
grv
r
r
r

ksfe
sekf
eksf
sfke
eskfy

vtilpszdyb
ytlizdps
yipslztfd
szxbdpvlyti

kafxuqswrmien
qsuwnrafmekix
zwgfrupkjvmnceatixd

inlodmzwqycgv
ncgorifdeuvm
opdnvfmgci
icdfpognvmajh
xnigcdeovpm

vsgtewndyruoij
oitwujsgpdren

jcxuoqsyrzfpt
uptfoqzyscirxj
zorfscpjxuytq

dpruh
drphu
wnuhpdr
puarhdti
hdurnp

kcafmguxdh
iorumvypkhd

hynfqaruk
fnqyukha
knqhfuay
anhyqkuf
aynuhqkf

ikawdgopncuvmblzqhrtyfe
vrtfclbegwnyhzmquokaipd
buachpomlzjywnfievrqtgdk
uthefikrpmzlncvyodabqwg
oadrvhmuzctkgfwlenqiybp

uixcszypmbvlnqkateofhwg
fhdexyupozjqtmgcanb

boszchprvjuweadykfg
krhxtezmosgacyfvp

kd
vemd

xuh
x
xcv
xu

pzbxkfnvo
kvxznf
kfpodwvxzn
znvfkxuh

pyuqlh
phlqu
pulhq

jnkzrsgbxeyqwmd
ygwcdosbekqxmnz

mvqx
mcguv

utvzrw
oqekh
trza

rj
rj
jrx
jr
jr

fzorkebhymi
zfhimejlkayr
gfrnmkstizqvey
uzmrfapijxwdeyk

jg
gj
gj
jg

tysdmhrj
xhmdrleft
hdtxmrq
rdhtm

l
l
l
l
l

fp
fxp

pmbcvrluqxkwhiasgdnefj
vdheksfxbgauilnctmwojqp

ueacivdmlbszfrjpwnxto
deprznxfumjtsoavibcl
imlytfvuxejcrsnkbopzda
ctenfoabslvxgrdmuzipj

vmyjs
wvcuzgn
tvefignd
dlehv

sndgvcbekuqfmlrihyo
ulfqhezoiydbgmscrnvk

xq
tq
q
q
q

vgjhnayrduxi
fpicvzsotqkdywhebjml

xi
ix
ix
ix
xi

idwkxjtemqfsvagnhluzobp
qghxukenilstbzopwavcfm
hfxqklvbogiwuamnpzste

eaiupytfqszdclwxo
dexuiczofwatlypqs
texlfzaswqpcouidy
tfzeaioulcwmqpysxd

vsfurzlgbiwnmq
vylzrdfgwencsqumb
vilsbnwfuqrzxgm
azgkbjfumpsvwlhqnr

f
l
mjtwk
y
e

qltizkeyus
slyqgtz
rptzqsly
zvlbqnajsytx
gzylutscqh

sorlqdakg
qyrkldv
qldws
fqdxe

uph
h

wvfyudbh
fdwvhy
vywhdf
vyfhdw

yxhfjmqes
yfjdusq

uoszjvmexyarchiw
rxjmucavyhos
ujhrcxmovays
cujxrvahnoslymp

iufbrj
uijfr

cdjboaziqvsgw
qvajzcwgdbios
pvqijagfcwobdzs
jbwczqgvadsoi

jqysrebxpoc
esqgopihyxkcrjb
yrevfxsoqpjbc
jpescbuqwroyx
znruxedyfcpbqosjm

lkbmeisofd
lmbiofdkes

amujfhrceb
frbhacujme
eubachfrmj
remhtcabjfu

owzke
axunbdhelk

dmalenfuypgistcvor
umeipogydfstraclvn
prytlgscofandiuvemx

zgifdbjmuoarnhs
qcrgujabftivdshk

wejh
j
fjezwv
rnjkibcmt
jo

guzs
rzgsu
zsuqgj
uzsg
guzs

tgrespwdkuqby
dykwuptrgebqs

znsudycxea
ucynzpdsex
unzdyxecs
dezyuscxn
ciuenszvyxd

htmbgcpwusvolade
bsavwugcqmehlopdnt
qalpwehtgoucmdsbv
stpbmgaelxhvkucdwo
gpbeacdmulhtswvro

kl
kl

vxzpadftihb
dzhvwpbtfaxi
tivaknsboxjeqrmfhgcp

es
e
zet
se

qdhfo
ybzh
yhbwjlm

hackjstyib
bckxdjihtsy
hsbklcyji
bgozhwncisvjpky
bhysjkci

qgdpftbsr
eqhbsoptgu
wlsjtpygxb
pabhsgt

uyndclr
tdlunryc
nluyrdc

olwfgvpyqcjt
vpjtfqlwgyo
vjoyqlwufpgat
woyqxptjlfgv
vtylqpofgwj

qgejcmyi
qbrkgumej

zjcbnvfgswh
fezijwshgcnvb
hvnyrxmzsqflwcbtgjo
fnwjsgbhzcv
hfjcgsnvbwze

ultbxowkch
lobxwchetk
loguhwcxtkb
awkhltxocb
wltohubzcxk

abtxuvg
abx
bxa
bsxa

bxdhufeil
tinhzg
aoyjih

guwvemnzxcpjk
jzmkuvypwgnei
nrjphszwukmbgeqv
ouwzekncdygvpmj
zgujwpenkmv

itpogznwyevbhj
nbehjsmytizwpvdo

crn
ncr
rcn
mcresna

efyvcqsmxbo
npjret

grywnhikfdeoatzm
jxluedwkmvqspfacb

jxmthefuvwndcr
bapkgsozrvyjmi

oifctylgbe
xphewqjrl

dpfsoljkemzuwyrcxhi
sphrujydiwxozlck
qxbjcoryawduzpiksvtlh
uokcsphixmjwrydlzf

csgkiwof
rmvqze
mpv
pjqx

vxdqtlrgcsmpzkbyiawo
vpskmytliacgwbdorqx
ragdkpbyqowtcsvlxe

xjednfipmaghzubkv
iebmuavsjzhgryxowkpf
itqphagefczxvnmjubk

x
xp
x
xgn

vh
jvh
hv
hv

rwe
akmop
lcnhusgivtfzjb
mowd

suyl
lusy
ylsu
lsyu
lursyn

htvjuspqlmwenybrkacfdo
tapfnyseilkucomrbvh

xidbpvsozrjtkuyhmln
ytjnvhbuldpmiksxroz
ubltzkvhonjdmispxry
lmnytjrxkbisopuhzdv
oibstukrdnyzpvhjlmx

r
r
r

slfcqvwdbimha
dasfmvqwiclhb
dmafqvicslwpbh
mwshilabvfqdc

loarezd
dluarzoe
zerdoavl
odleagzr

rvbjxn
jvnbxwr
ofnxgbjzr
xrwbjn
bxjren

q
jqe
jq
qypzc

bneacsmtji
nsbjceftamiy
baswehcpmnti
stieanmbc

vnms
tkcjoaxuyw
dnlg
idpv

hxjsiancvugpedmrzfoyqtwlkb
qxalgitvjhosrwukycmfebdnp
ckhirotldgpusfwveymqbaxnj
axmtiocnsfbqwgevdypjrulhk
hepjdtbfkicoamrlwnyqvgusx

aqmxcepog
qoemcagp
cmwgaoqpe
zedcohgsmlkaqp

wuryjmckgoa
vukowjgay
guoywzaejfkxt

vqcnxmflhjzpr
hpjelxqkbtcvnms

q
vt

cpfwuydnitjsgxbhzvormk
imspaweoqgkcfuldvxhrb

icaj
ciaj
cjia

xhmleqfridavt
tkdixyqavmelfhr
talhdmvfeqirx

jaytvesdu
bfnqeagirp
zelaj

lvbapwkxyigheqts
eywqpszvxtkaghli
ytlxvghkqeiwszp
yquhecpnjlidtkgxvrws

bjn
nbj

vxyjehaou
wozap
";
