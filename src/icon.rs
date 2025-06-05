use crate::color::COLORS;
use crate::os::Distro;

impl Distro {
    pub fn icon(&self) -> String {
        match self {
            Distro::Aix => aix(),
            Distro::AlmaLinux => alma_linux(),
            Distro::AlpineSmall => alpine_small(),
            Distro::Alpine => alpine(),
            Distro::Alter => alter(),
            Distro::Amazon => amazon(),
            Distro::Anarchy => anarchy(),
            Distro::AndroidSmall => android_small(),
            Distro::Android => android(),
            Distro::InstantOs => instant_os(),
            Distro::Antergos => antergos(),
            Distro::AntiX => anti_x(),
            Distro::AoscOs => aosc_os(),
            Distro::Apricity => apricity(),
            Distro::Archcraft => archcraft(),
            Distro::ArcolinuxSmall => arcolinux_small(),
            Distro::ArcoLinux => arco_linux(),
            Distro::ArchSmall => arch_small(),
            Distro::ArchOld => arch_old(),
            Distro::ArchBox => arch_box(),
            Distro::ArcHlabs => arc_hlabs(),
            Distro::ArchStrike => arch_strike(),
            Distro::XFerience => x_ferience(),
            Distro::ArchMerge => arch_merge(),
            Distro::Arch => arch(),
            Distro::ArtixSmall => artix_small(),
            Distro::Artix => artix(),
            Distro::Arya => arya(),
            Distro::Bedrock => bedrock(),
            Distro::Bitrig => bitrig(),
            Distro::BlackArch => black_arch(),
            Distro::Blag => blag(),
            Distro::BlankOn => blank_on(),
            Distro::BlueLight => blue_light(),
            Distro::Bonsai => bonsai(),
            Distro::BunsenLabs => bunsen_labs(),
            Distro::Calculate => calculate(),
            Distro::Carbs => carbs(),
            Distro::CblMariner => cbl_mariner(),
            Distro::CelOs => cel_os(),
            Distro::CentosSmall => centos_small(),
            Distro::CentOs => cent_os(),
            Distro::Chakra => chakra(),
            Distro::ChaletOs => chalet_os(),
            Distro::Chapeau => chapeau(),
            Distro::Chrom => chrom(),
            Distro::CleanjaroSmall => cleanjaro_small(),
            Distro::Cleanjaro => cleanjaro(),
            Distro::ClearOs => clear_os(),
            Distro::ClearLinuxOs => clear_linux_os(),
            Distro::Clover => clover(),
            Distro::Condres => condres(),
            Distro::ContainerLinuxByCoreOs => container_linux_by_core_os(),
            Distro::CruxSmall => crux_small(),
            Distro::Crux => crux(),
            Distro::CrystalLinux => crystal_linux(),
            Distro::Cucumber => cucumber(),
            Distro::Dahlia => dahlia(),
            Distro::DebianSmall => debian_small(),
            Distro::Debian => debian(),
            Distro::Deepin => deepin(),
            Distro::DesaOs => desa_os(),
            Distro::Devuan => devuan(),
            Distro::DracOs => drac_os(),
            Distro::Itc => itc(),
            Distro::DragonflyOld => dragonfly_old(),
            Distro::DragonflySmall => dragonfly_small(),
            Distro::DragonFly => dragon_fly(),
            Distro::Drauger => drauger(),
            Distro::ElementarySmall => elementary_small(),
            Distro::Elementary => elementary(),
            Distro::EndeavourOs => endeavour_os(),
            Distro::Endless => endless(),
            Distro::EuroLinux => euro_linux(),
            Distro::Exherbo => exherbo(),
            Distro::FedoraSmall => fedora_small(),
            Distro::FedoraOld => fedora_old(),
            Distro::Fedora => fedora(),
            Distro::Feren => feren(),
            Distro::FreebsdSmall => freebsd_small(),
            Distro::FreeMiNt => free_mi_nt(),
            Distro::Frugalware => frugalware(),
            Distro::Funtoo => funtoo(),
            Distro::GalliumOs => gallium_os(),
            Distro::Garuda => garuda(),
            Distro::GentooSmall => gentoo_small(),
            Distro::Gentoo => gentoo(),
            Distro::Pentoo => pentoo(),
            Distro::Glaucus => glaucus(),
            Distro::GNewSense => g_new_sense(),
            Distro::Gnome => gnome(),
            Distro::Gnu => gnu(),
            Distro::GoboLinux => gobo_linux(),
            Distro::Grombyang => grombyang(),
            Distro::GuixSmall => guix_small(),
            Distro::Guix => guix(),
            Distro::HaikuSmall => haiku_small(),
            Distro::Haiku => haiku(),
            Distro::Huayra => huayra(),
            Distro::HydroOs => hydro_os(),
            Distro::HyperbolaSmall => hyperbola_small(),
            Distro::Hyperbola => hyperbola(),
            Distro::Iglunix => iglunix(),
            Distro::Januslinux => januslinux(),
            Distro::Kaisen => kaisen(),
            Distro::Kali => kali(),
            Distro::KaOs => ka_os(),
            Distro::Kde => kde(),
            Distro::Kibojoe => kibojoe(),
            Distro::Kogaion => kogaion(),
            Distro::Korora => korora(),
            Distro::KsLinux => ks_linux(),
            Distro::Kubuntu => kubuntu(),
            Distro::Lede => lede(),
            Distro::LaxerOs => laxer_os(),
            Distro::LibreElec => libre_elec(),
            Distro::Linux => linux(),
            Distro::LinuxliteSmall => linuxlite_small(),
            Distro::LinuxLite => linux_lite(),
            Distro::Lmde => lmde(),
            Distro::Lubuntu => lubuntu(),
            Distro::Lunar => lunar(),
            Distro::Mac => mac(),
            Distro::MageiaSmall => mageia_small(),
            Distro::Mageia => mageia(),
            Distro::MagpieOs => magpie_os(),
            Distro::Mandriva => mandriva(),
            Distro::ManjaroSmall => manjaro_small(),
            Distro::Manjaro => manjaro(),
            Distro::Maui => maui(),
            Distro::Mer => mer(),
            Distro::Minix => minix(),
            Distro::LinuxmintSmall => linuxmint_small(),
            Distro::LinuxMintOld => linux_mint_old(),
            Distro::LinuxMint => linux_mint(),
            Distro::LiveRaizo => live_raizo(),
            Distro::MxSmall => mx_small(),
            Distro::Mx => mx(),
            Distro::Namib => namib(),
            Distro::Neptune => neptune(),
            Distro::NetbsdSmall => netbsd_small(),
            Distro::NetBsd => net_bsd(),
            Distro::Netrunner => netrunner(),
            Distro::Nitrux => nitrux(),
            Distro::NixosSmall => nixos_small(),
            Distro::NixosOld => nixos_old(),
            Distro::NixOs => nix_os(),
            Distro::Nurunner => nurunner(),
            Distro::NuTyX => nu_ty_x(),
            Distro::ObRevenge => ob_revenge(),
            Distro::OpenbsdSmall => openbsd_small(),
            Distro::OpenBsd => open_bsd(),
            Distro::OpenEuler => open_euler(),
            Distro::OpenIndiana => open_indiana(),
            Distro::Openmamba => openmamba(),
            Distro::OpenMandriva => open_mandriva(),
            Distro::OpenStage => open_stage(),
            Distro::OpenWrt => open_wrt(),
            Distro::OpenSourceMediaCenter => open_source_media_center(),
            Distro::Oracle => oracle(),
            Distro::OsElbrus => os_elbrus(),
            Distro::PacBsd => pac_bsd(),
            Distro::ParabolaSmall => parabola_small(),
            Distro::Parabola => parabola(),
            Distro::Pardus => pardus(),
            Distro::Parrot => parrot(),
            Distro::Parsix => parsix(),
            Distro::Pcbsd => pcbsd(),
            Distro::PcLinuxOs => pc_linux_os(),
            Distro::Pengwin => pengwin(),
            Distro::Peppermint => peppermint(),
            Distro::PoposSmall => popos_small(),
            Distro::PopOs => pop_os(),
            Distro::Porteus => porteus(),
            Distro::PostmarketosSmall => postmarketos_small(),
            Distro::PostMarketOs => post_market_os(),
            Distro::PuffOs => puff_os(),
            Distro::Puppy => puppy(),
            Distro::PureosSmall => pureos_small(),
            Distro::PureOs => pure_os(),
            Distro::Qubes => qubes(),
            Distro::Qubyt => qubyt(),
            Distro::Quibian => quibian(),
            Distro::Radix => radix(),
            Distro::RaspbianSmall => raspbian_small(),
            Distro::Raspbian => raspbian(),
            Distro::RebornOs => reborn_os(),
            Distro::RedStar => red_star(),
            Distro::Redcore => redcore(),
            Distro::RedhatOld => redhat_old(),
            Distro::Redhat => redhat(),
            Distro::RefractedDevuan => refracted_devuan(),
            Distro::Regata => regata(),
            Distro::Regolith => regolith(),
            Distro::RockySmall => rocky_small(),
            Distro::Rosa => rosa(),
            Distro::Sabotage => sabotage(),
            Distro::Sabayon => sabayon(),
            Distro::Sailfish => sailfish(),
            Distro::SalentOs => salent_os(),
            Distro::Scientific => scientific(),
            Distro::Septor => septor(),
            Distro::Serene => serene(),
            Distro::SharkLinux => shark_linux(),
            Distro::SlackwareSmall => slackware_small(),
            Distro::Slackware => slackware(),
            Distro::SliTaz => sli_taz(),
            Distro::SmartOs => smart_os(),
            Distro::Solus => solus(),
            Distro::SourceMage => source_mage(),
            Distro::Sparky => sparky(),
            Distro::Star => star(),
            Distro::SteamOs => steam_os(),
            Distro::SunosSmall => sunos_small(),
            Distro::OpenSuseLeap => open_suse_leap(),
            Distro::T2 => t2(),
            Distro::OpenSuseTumbleweed => open_suse_tumbleweed(),
            Distro::OpensuseSmall => opensuse_small(),
            Distro::OpenSuse => open_suse(),
            Distro::SwagArch => swag_arch(),
            Distro::Tails => tails(),
            Distro::Trisquel => trisquel(),
            Distro::UbuntuCinnamon => ubuntu_cinnamon(),
            Distro::UbuntuBudgie => ubuntu_budgie(),
            Distro::UbuntuGnome => ubuntu_gnome(),
            Distro::UbuntuMate => ubuntu_mate(),
            Distro::UbuntuOld => ubuntu_old(),
            Distro::UbuntuStudio => ubuntu_studio(),
            Distro::UbuntuSmall => ubuntu_small(),
            Distro::Ubuntu => ubuntu(),
            Distro::Univention => univention(),
            Distro::Venom => venom(),
            Distro::VoidSmall => void_small(),
            Distro::LangitKetujuh => langit_ketujuh(),
            Distro::Semc => semc(),
            Distro::Obarun => obarun(),
            Distro::Windows11 => windows11(),
            Distro::Windows10 => windows10(),
            Distro::Windows => windows(),
            Distro::Xubuntu => xubuntu(),
            Distro::Zorin => zorin(),
            Distro::Darwin => darwin(),
            Distro::ProfelisSambaBox => profelis_samba_box(),
            Distro::Unknown => String::new(),
        }
    }
}

pub fn aix() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}           `:+ssssossossss+-`
        .oys///oyhddddhyo///sy+.
      /yo:+hNNNNNNNNNNNNNNNNh+:oy/
    :h/:yNNNNNNNNNNNNNNNNNNNNNNy-+h:
  `ys.yNNNNNNNNNNNNNNNNNNNNNNNNNNy.ys
 `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oh
 h+-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN.oy
/d`mNNNNNNN/::mNNNd::m+:/dNNNo::dNNNd`m:
h//NNNNNNN: . .NNNh  mNo  od. -dNNNNN:+y
N.sNNNNNN+ -N/ -NNh  mNNd.   sNNNNNNNo-m
N.sNNNNNs  +oo  /Nh  mNNs` ` /mNNNNNNo-m
h//NNNNh  ossss` +h  md- .hm/ `sNNNNN:+y
:d`mNNN+/yNNNNNd//y//h//oNNNNy//sNNNd`m-
 yo-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNm.ss
 `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oy
   sy.yNNNNNNNNNNNNNNNNNNNNNNNNNNs.yo
    :h+-yNNNNNNNNNNNNNNNNNNNNNNs-oh-
      :ys:/yNNNNNNNNNNNNNNNmy/:sy:
        .+ys///osyhhhhys+///sy+.
            -/osssossossso/-"##########
    )
}

// pub fn Hash() -> String {
//     let c1 = COLORS[123];
//     format!(
//         r##########"{c1}

//       +   ######   +
//     ###   ######   ###
//   #####   ######   #####
//  ######   ######   ######

// ####### '"###### '"########
// #######   ######   ########
// #######   ######   ########

//  ###### '"###### '"######
//   #####   ######   #####
//     ###   ######   ###
//       ~   ######   ~"##########
//     )
// }

pub fn alma_linux() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[3];
    let c3 = COLORS[4];
    let c4 = COLORS[2];
    let c5 = COLORS[6];
    format!(
        r##########"{c1}         'c:.
{c1}        lkkkx, ..       {c2}..   ,cc,
{c1}        okkkk:ckkx'  {c2}.lxkkx.okkkkd
{c1}        .:llcokkx'  {c2}:kkkxkko:xkkd,
{c1}      .xkkkkdood:  {c2};kx,  .lkxlll;
{c1}       xkkx.       {c2}xk'     xkkkkk:
{c1}       'xkx.       {c2}xd      .....,.
{c3}      .. {c1}:xkl'     {c2}:c      ..''..
{c3}    .dkx'  {c1}.:ldl:'. {c2}'  {c4}':lollldkkxo;
{c3}  .''lkkko'                     {c4}ckkkx.
{c3}'xkkkd:kkd.       ..  {c5};'        {c4}:kkxo.
{c3},xkkkd;kk'      ,d;    {c5}ld.   {c4}':dkd::cc,
{c3} .,,.;xkko'.';lxo.      {c5}dx,  {c4}:kkk'xkkkkc
{c3}     'dkkkkkxo:.        {c5};kx  {c4}.kkk:;xkkd.
{c3}       .....   {c5}.;dk:.   {c5}lkk.  {c4}:;,
             {c5}:kkkkkkkdoxkkx
              ,c,,;;;:xkkd.
                ;kkkkl...
                ;kkkkl
                 ,od;"##########
    )
}

pub fn alpine_small() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}   /\\ /\\
  /{c2}/ {c1}\\  \\
 /{c2}/   {c1}\\  \\
/{c2}//    {c1}\\  \\
{c2}//      {c1}\\  \\
         \\"##########
    )
}

pub fn alpine() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}       .hddddddddddddddddddddddh.
      :dddddddddddddddddddddddddd:
     /dddddddddddddddddddddddddddd/
    +dddddddddddddddddddddddddddddd+
  `sdddddddddddddddddddddddddddddddds`
 `ydddddddddddd++hdddddddddddddddddddy`
.hddddddddddd+`  `+ddddh:-sdddddddddddh.
hdddddddddd+`      `+y:    .sddddddddddh
ddddddddh+`   `//`   `.`     -sddddddddd
ddddddh+`   `/hddh/`   `:s-    -sddddddd
ddddh+`   `/+/dddddh/`   `+s-    -sddddd
ddd+`   `/o` :dddddddh/`   `oy-    .yddd
hdddyo+ohddyosdddddddddho+oydddy++ohdddh
.hddddddddddddddddddddddddddddddddddddh.
 `yddddddddddddddddddddddddddddddddddy`
  `sdddddddddddddddddddddddddddddddds`
    +dddddddddddddddddddddddddddddd+
     /dddddddddddddddddddddddddddd/
      :dddddddddddddddddddddddddd:
       .hddddddddddddddddddddddh."##########
    )
}

pub fn alter() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                      %,
                    ^WWWw
                   'wwwwww
                  !wwwwwwww
                 #`wwwwwwwww
                @wwwwwwwwwwww
               wwwwwwwwwwwwwww
              wwwwwwwwwwwwwwwww
             wwwwwwwwwwwwwwwwwww
            wwwwwwwwwwwwwwwwwwww,
           w~1i.wwwwwwwwwwwwwwwww,
         3~:~1lli.wwwwwwwwwwwwwwww.
        :~~:~?ttttzwwwwwwwwwwwwwwww
       #<~:~~~~?llllltO-.wwwwwwwwwww
      #~:~~:~:~~?ltlltlttO-.wwwwwwwww
     @~:~~:~:~:~~(zttlltltlOda.wwwwwww
    @~:~~: ~:~~:~:(zltlltlO    a,wwwwww
   8~~:~~:~~~~:~~~~_1ltltu          ,www
  5~~:~~:~~:~~:~~:~~~_1ltq             N,,
 g~:~~:~~~:~~:~~:~:~~~~1q                N,"##########
    )
}

pub fn amazon() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}             `-/oydNNdyo:.`
      `.:+shmMMMMMMMMMMMMMMmhs+:.`
    -+hNNMMMMMMMMMMMMMMMMMMMMMMNNho-
.``      -/+shmNNMMMMMMNNmhs+/-      ``.
dNmhs+:.       `.:/oo/:.`       .:+shmNd
dMMMMMMMNdhs+:..        ..:+shdNMMMMMMMd
dMMMMMMMMMMMMMMNds    odNMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
.:+ydNMMMMMMMMMMMh    yMMMMMMMMMMMNdy+:.
     `.:+shNMMMMMh    yMMMMMNhs+:``
            `-+shy    shs+:`"##########
    )
}

pub fn anarchy() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[4];
    format!(
        r##########"{c2}..{c1}
                        {c2}..{c1}
                      {c2}:..{c1}
                    {c2}:+++.{c1}
              .:::++{c2}++++{c1}+::.
          .:+######{c2}++++{c1}######+:.
       .+#########{c2}+++++{c1}##########:.
     .+##########{c2}+++++++{c1}##{c2}+{c1}#########+.
    +###########{c2}+++++++++{c1}############:
   +##########{c2}++++++{c1}#{c2}++++{c1}#{c2}+{c1}###########+
  +###########{c2}+++++{c1}###{c2}++++{c1}#{c2}+{c1}###########+
 :##########{c2}+{c1}#{c2}++++{c1}####{c2}++++{c1}#{c2}+{c1}############:
 ###########{c2}+++++{c1}#####{c2}+++++{c1}#{c2}+{c1}###{c2}++{c1}######+
.##########{c2}++++++{c1}#####{c2}++++++++++++{c1}#######.
.##########{c2}+++++++++++++++++++{c1}###########.
 #####{c2}++++++++++++++{c1}###{c2}++++++++{c1}#########+
 :###{c2}++++++++++{c1}#########{c2}+++++++{c1}#########:
  +######{c2}+++++{c1}##########{c2}++++++++{c1}#######+
   +####{c2}+++++{c1}###########{c2}+++++++++{c1}#####+
    :##{c2}++++++{c1}############{c2}++++++++++{c1}##:
     .{c2}++++++{c1}#############{c2}++++++++++{c1}+.
      :{c2}++++{c1}###############{c2}+++++++{c1}::
     .{c2}++. .:+{c1}##############{c2}+++++++{c1}..
     {c2}.:.{c1}      ..::++++++::..:{c2}++++{c1}+.
     {c2}.{c1}                       {c2}.:+++{c1}.
                                {c2}.:{c1}:
                                   {c2}..{c1}
                                    {c2}..{c1}"##########
    )
}

pub fn android_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}  ;,           ,;
   ';,.-----.,;'
  ,'           ',
 /    O     O    \\
|                 |
'-----------------'"##########
    )
}

pub fn android() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}         -o          o-
          +hydNNNNdyh+
        +mMMMMMMMMMMMMm+
      `dMM{c2}m:{c1}NMMMMMMN{c2}:m{c1}MMd`
      hMMMMMMMMMMMMMMMMMMh
  ..  yyyyyyyyyyyyyyyyyyyy  ..
.mMMm`MMMMMMMMMMMMMMMMMMMM`mMMm.
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
-MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM-
 +yy+ MMMMMMMMMMMMMMMMMMMM +yy+
      mMMMMMMMMMMMMMMMMMMm
      `/++MMMMh++hMMMM++/`
          MMMMo  oMMMM
          MMMMo  oMMMM
          oNMm-  -mMNs"##########
    )
}

pub fn instant_os() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}
     'cx0XWWMMWNKOd:'.
  .;kNMMMMMMMMMMMMMWNKd'
 'kNMMMMMMWNNNWMMMMMMMMXo.
,0MMMMMW0o;'..,:dKWMMMMMWx.
OMMMMMXl.        .xNMMMMMNo
WMMMMNl           .kWWMMMMO'
MMMMMX;            oNWMMMMK,
NMMMMWo           .OWMMMMMK,
kWMMMMNd.        ,kWMMMMMMK,
'kWMMMMWXxl:;;:okNMMMMMMMMK,
 .oXMMMMMMMWWWMMMMMMMMMMMMK,
   'oKWMMMMMMMMMMMMMMMMMMMK,
     .;lxOKXXXXXXXXXXXXXXXO;......
          ................,d0000000kd:.
                          .kMMMMMMMMMW0;
                          .kMMMMMMMMMMMX
                          .xMMMMMMMMMMMW
                           cXMMMMMMMMMM0
                            :0WMMMMMMNx,
                             .o0NMWNOc."##########
    )
}

pub fn antergos() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[6];
    format!(
        r##########"{c2}              `.-/::/-``
            .-/osssssssso/.
           :osyysssssssyyys+-
        `.+yyyysssssssssyyyyy+.
       `/syyyyyssssssssssyyyyys-`
      `/yhyyyyysss{c1}++{c2}ssosyyyyhhy/`
     .ohhhyyyys{c1}o++/+o{c2}so{c1}+{c2}syy{c1}+{c2}shhhho.
    .shhhhys{c1}oo++//+{c2}sss{c1}+++{c2}yyy{c1}+s{c2}hhhhs.
   -yhhhhs{c1}+++++++o{c2}ssso{c1}+++{c2}yyy{c1}s+o{c2}hhddy:
  -yddhhy{c1}o+++++o{c2}syyss{c1}++++{c2}yyy{c1}yooy{c2}hdddy-
 .yddddhs{c1}o++o{c2}syyyyys{c1}+++++{c2}yyhh{c1}sos{c2}hddddy`
`odddddhyosyhyyyyyy{c1}++++++{c2}yhhhyosddddddo
.dmdddddhhhhhhhyyyo{c1}+++++{c2}shhhhhohddddmmh.
ddmmdddddhhhhhhhso{c1}++++++{c2}yhhhhhhdddddmmdy
dmmmdddddddhhhyso{c1}++++++{c2}shhhhhddddddmmmmh
-dmmmdddddddhhys{c1}o++++o{c2}shhhhdddddddmmmmd-
.smmmmddddddddhhhhhhhhhdddddddddmmmms.
   `+ydmmmdddddddddddddddddddmmmmdy/.
      `.:+ooyyddddddddddddyyso+:.`"##########
    )
}

pub fn anti_x() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}
                    \
         , - ~ ^ ~ - \        /
     , '              \ ' ,  /
   ,                   \   '/
  ,                     \  / ,
 ,___,                   \/   ,
 /   |   _  _  _|_ o     /\   ,
|,   |  / |/ |  |  |    /  \  ,
 \,_/\_/  |  |_/|_/|_/_/    \,
   ,                  /     ,\
     ,               /  , '   \
      ' - , _ _ _ ,  '"##########
    )
}

pub fn aosc_os() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}             .:+syhhhhys+:.
         .ohNMMMMMMMMMMMMMMNho.
      `+mMMMMMMMMMMmdmNMMMMMMMMm+`
     +NMMMMMMMMMMMM/   `./smMMMMMN+
   .mMMMMMMMMMMMMMMo        -yMMMMMm.
  :NMMMMMMMMMMMMMMMs          .hMMMMN:
 .NMMMMhmMMMMMMMMMMm+/-         oMMMMN.
 dMMMMs  ./ymMMMMMMMMMMNy.       sMMMMd
-MMMMN`      oMMMMMMMMMMMN:      `NMMMM-
/MMMMh       NMMMMMMMMMMMMm       hMMMM/
/MMMMh       NMMMMMMMMMMMMm       hMMMM/
-MMMMN`      :MMMMMMMMMMMMy.     `NMMMM-
 dMMMMs       .yNMMMMMMMMMMMNy/. sMMMMd
 .NMMMMo         -/+sMMMMMMMMMMMmMMMMN.
  :NMMMMh.          .MMMMMMMMMMMMMMMN:
   .mMMMMMy-         NMMMMMMMMMMMMMm.
     +NMMMMMms/.`    mMMMMMMMMMMMN+
      `+mMMMMMMMMNmddMMMMMMMMMMm+`
         .ohNMMMMMMMMMMMMMMNho.
             .:+syhhhhys+:."##########
    )
}

pub fn apricity() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                                    ./o-
          ``...``              `:. -/:
     `-+ymNMMMMMNmho-`      :sdNNm/
   `+dMMMMMMMMMMMMMMMmo` sh:.:::-
  /mMMMMMMMMMMMMMMMMMMMm/`sNd/
 oMMMMMMMMMMMMMMMMMMMMMMMs -`
:MMMMMMMMMMMMMMMMMMMMMMMMM/
NMMMMMMMMMMMMMMMMMMMMMMMMMd
MMMMMMMmdmMMMMMMMMMMMMMMMMd
MMMMMMy` .mMMMMMMMMMMMmho:`
MMMMMMNo/sMMMMMMMNdy+-.`-/
MMMMMMMMMMMMNdy+:.`.:ohmm:
MMMMMMMmhs+-.`.:+ymNMMMy.
MMMMMM/`.-/ohmNMMMMMMy-
MMMMMMNmNNMMMMMMMMmo.
MMMMMMMMMMMMMMMms:`
MMMMMMMMMMNds/.
dhhyys+/-`"##########
    )
}

pub fn archcraft() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                   -m:
                  :NMM+      .+
                 +MMMMMo    -NMy
                sMMMMMMMy  -MMMMh`
               yMMMMMMMMMd` oMMMMd`
             `dMMMMMMMMMMMm. /MMMMm-
            .mMMMMMm-dMMMMMN- :NMMMN:
           -NMMMMMd`  yMMMMMN: .mMMMM/
          :NMMMMMy     sMMMMMM+ `dMMMMo
         +MMMMMMs       +MMMMMMs `hMMMMy
        oMMMMMMMds-      :NMMMMMy  sMMMMh`
       yMMMMMNoydMMmo`    -NMMMMMd` +MMMMd.
     `dMMMMMN-   `:yNNs`   .mMMMMMm. /MMMMm-
    .mMMMMMm.        :hN/   `dMMMMMN- -NMMMN:
   -NMMMMMd`           -hh`  `yMMMMMN: .mMMMM/
  :NMMMMMy         `s`   :h.   oMMMMMM+ `-----
 +MMMMMMo         .dMm.   `o.   +MMMMMMo
sMMMMMM+         .mMMMN:    :`   :NMMMMMy"##########
    )
}

pub fn arcolinux_small() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[4];
    format!(
        r##########"{c2}          A
         ooo
        ooooo
       ooooooo
      ooooooooo
     ooooo ooooo
    ooooo   ooooo
   ooooo     ooooo
  ooooo  {c1}<oooooooo>{c2}
 ooooo      {c1}<oooooo>{c2}
ooooo          {c1}<oooo>{c2}"##########
    )
}

pub fn arco_linux() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[4];
    format!(
        r##########"{c2}                    /-
                   ooo:
                  yoooo/
                 yooooooo
                yooooooooo
               yooooooooooo
             .yooooooooooooo
            .oooooooooooooooo
           .oooooooarcoooooooo
          .ooooooooo-oooooooooo
         .ooooooooo-  oooooooooo
        :ooooooooo.    :ooooooooo
       :ooooooooo.      :ooooooooo
      :oooarcooo         .oooarcooo
     :ooooooooy           .ooooooooo
    :ooooooooo   {c1}/ooooooooooooooooooo{c2}
   :ooooooooo      {c1}.-ooooooooooooooooo.{c2}
  ooooooooo-             {c1}-ooooooooooooo.{c2}
 ooooooooo-                 {c1}.-oooooooooo.{c2}
ooooooooo.                     {c1}-ooooooooo{c2}"##########
    )
}

pub fn arch_small() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}      /\\
     /  \\
    /\\   \\
{c2}   /      \\
  /   ,,   \\
 /   |  |  -\\
/_-''    ''-_\\"##########
    )
}

pub fn arch_old() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}             __
         _=(SDGJT=_
       _GTDJHGGFCVS)
      ,GTDJGGDTDFBGX0
{c1}     JDJDIJHRORVFSBSVL{c2}-=+=,_
{c1}    IJFDUFHJNXIXCDXDSV,{c2}  "DEBL
{c1}   [LKDSDJTDU=OUSCSBFLD.{c2}   '?ZWX,
{c1}  ,LMDSDSWH'     `DCBOSI{c2}     DRDS],
{c1}  SDDFDFH'         !YEWD,{c2}   )HDROD
{c1} !KMDOCG            &GSU|{c2}\_GFHRGO\'
{c1} HKLSGP'{c2}           __{c1}\TKM0{c2}\GHRBV)'
{c1}JSNRVW'{c2}       __+MNAEC{c1}\IOI,{c2}\BN'
{c1}HELK['{c2}    __,=OFFXCBGHC{c1}\FD)
{c1}?KGHE {c2}\_-#DASDFLSV='{c1}    'EF
'EHTI                    !H
 `0F'                    '!"##########
    )
}

pub fn arch_box() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}              ...:+oh/:::..
         ..-/oshhhhhh`   `::::-.
     .:/ohhhhhhhhhhhh`        `-::::.
 .+shhhhhhhhhhhhhhhhh`             `.::-.
 /`-:+shhhhhhhhhhhhhh`            .-/+shh
 /      .:/ohhhhhhhhh`       .:/ohhhhhhhh
 /           `-:+shhh`  ..:+shhhhhhhhhhhh
 /                 .:ohhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /      .+o+        `hhhhhhhhhhhhhhhhhhhh
 /     -hhhhh       `hhhhhhhhhhhhhhhhhhhh
 /     ohhhhho      `hhhhhhhhhhhhhhhhhhhh
 /:::+`hhhhoos`     `hhhhhhhhhhhhhhhhhs+`
    `--/:`   /:     `hhhhhhhhhhhho/-
             -/:.   `hhhhhhs+:-`
                ::::/ho/-`"##########
    )
}

pub fn arc_hlabs() -> String {
    let c1 = COLORS[6];
    let c4 = COLORS[1];
    format!(
        r##########"{c1}                     'c'
                    'kKk,
                   .dKKKx.
                  .oKXKXKd.
                 .l0XXXXKKo.
                 c0KXXXXKX0l.
                :0XKKOxxOKX0l.
               :OXKOc. .c0XX0l.
              :OK0o. {c4}...{c1}'dKKX0l.
             :OX0c  {c4};xOx'{c1}'dKXX0l.
            :0KKo.{c4}.o0XXKd'.{c1}lKXX0l.
           c0XKd.{c4}.oKXXXXKd..{c1}oKKX0l.
         .c0XKk;{c4}.l0K0OO0XKd..{c1}oKXXKo.
        .l0XXXk:{c4},dKx,.'l0XKo.{c1}.kXXXKo.
       .o0XXXX0d,{c4}:x;   .oKKx'{c1}.dXKXXKd.
      .oKXXXXKK0c.{c4};.    :00c'{c1}cOXXXXXKd.
     .dKXXXXXXXXk,{c4}.     cKx'{c1}'xKXXXXXXKx'
    'xKXXXXK0kdl:.     {c4}.ok; {c1}.cdk0KKXXXKx'
   'xKK0koc,..         {c4}'c, {c1}    ..,cok0KKk,
  ,xko:'.             {c4}.. {c1}           .':okx;
 .,'.                                   .',."##########
    )
}

pub fn arch_strike() -> String {
    let c1 = COLORS[8];
    let c2 = COLORS[6];
    format!(
        r##########"{c1}                   *   
                  **.
                 ****
                ******
                *******
              ** *******
             **** *******
            {c1}****{c2}_____{c1}***{c2}/{c1}*
           ***{c2}/{c1}*******{c2}//{c1}***
          **{c2}/{c1}********{c2}///{c1}*{c2}/{c1}**
         **{c2}/{c1}*******{c2}////{c1}***{c2}/{c1}**
        **{c2}/{c1}****{c2}//////.,{c1}****{c2}/{c1}**
       ***{c2}/{c1}*****{c2}/////////{c1}**{c2}/{c1}***
      ****{c2}/{c1}****    {c2}/////{c1}***{c2}/{c1}****
     ******{c2}/{c1}***  {c2}////   {c1}**{c2}/{c1}******
    ********{c2}/{c1}* {c2}///      {c1}*{c2}/{c1}********
  ,******     {c2}// ______ /    {c1}******,"##########
    )
}

pub fn x_ferience() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}           ``--:::::::-.`
        .-/+++ooooooooo+++:-`
     `-/+oooooooooooooooooo++:.
    -/+oooooo/+ooooooooo+/ooo++:`
  `/+oo++oo.   .+oooooo+.-: +:-o+-
 `/+o/.  -o.    :oooooo+ ```:.+oo+-
`:+oo-    -/`   :oooooo+ .`-`+oooo/.
.+ooo+.    .`   `://///+-+..oooooo+:`
-+ooo:`                ``.-+oooooo+/`
-+oo/`                       :+oooo/.
.+oo:            ..-/. .      -+oo+/`
`/++-         -:::++::/.      -+oo+-
 ./o:          `:///+-     `./ooo+:`
  .++-         `` /-`   -:/+oooo+:`
   .:+/:``          `-:ooooooo++-
     ./+o+//:...../+oooooooo++:`
       `:/++ooooooooooooo++/-`
          `.-//++++++//:-.`
               ``````"##########
    )
}

pub fn arch_merge() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                    y:
                  sMN-
                 +MMMm`
                /MMMMMd`
               :NMMMMMMy
              -NMMMMMMMMs
             .NMMMMMMMMMM+
            .mMMMMMMMMMMMM+
            oNMMMMMMMMMMMMM+
          `+:-+NMMMMMMMMMMMM+
          .sNMNhNMMMMMMMMMMMM/
        `hho/sNMMMMMMMMMMMMMMM/
       `.`omMMmMMMMMMMMMMMMMMMM+
      .mMNdshMMMMd+::oNMMMMMMMMMo
     .mMMMMMMMMM+     `yMMMMMMMMMs
    .NMMMMMMMMM/        yMMMMMMMMMy
   -NMMMMMMMMMh         `mNMMMMMMMMd`
  /NMMMNds+:.`             `-/oymMMMm.
 +Mmy/.                          `:smN:
/+.                                  -o."##########
    )
}

pub fn arch() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[6];
    format!(
        r##########"{c1}                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++o{c2}oooooooo{c1}oooo/`
{c2}         {c1}./{c2}ooosssso++osssssso{c1}+`
{c2}        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/"##########
    )
}

pub fn artix_small() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}      /\\
     /  \\
    /`'.,\\
   /     ',
  /      ,`\\
 /   ,.'`.  \\
/.,'`     `'.\\"##########
    )
}

pub fn artix() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                   '
                  'o'
                 'ooo'
                'ooxoo'
               'ooxxxoo'
              'oookkxxoo'
             'oiioxkkxxoo'
            ':;:iiiioxxxoo'
               `'.;::ioxxoo'
          '-.      `':;jiooo'
         'oooio-..     `'i:io'
        'ooooxxxxoio:,.   `'-;'
       'ooooxxxxxkkxoooIi:-.  `'
      'ooooxxxxxkkkkxoiiiiiji'
     'ooooxxxxxkxxoiiii:'`     .i'
    'ooooxxxxxoi:::'`       .;ioxo'
   'ooooxooi::'`         .:iiixkxxo'
  'ooooi:'`                `'';ioxxo'
 'i:'`                          '':io'
'`                                   `'"##########
    )
}

pub fn arya() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[1];
    format!(
        r##########"{c1}                `oyyy/{c2}-yyyyyy+
{c1}               -syyyy/{c2}-yyyyyy+
{c1}              .syyyyy/{c2}-yyyyyy+
{c1}              :yyyyyy/{c2}-yyyyyy+
{c1}           `/ :yyyyyy/{c2}-yyyyyy+
{c1}          .+s :yyyyyy/{c2}-yyyyyy+
{c1}         .oys :yyyyyy/{c2}-yyyyyy+
{c1}        -oyys :yyyyyy/{c2}-yyyyyy+
{c1}       :syyys :yyyyyy/{c2}-yyyyyy+
{c1}      /syyyys :yyyyyy/{c2}-yyyyyy+
{c1}     +yyyyyys :yyyyyy/{c2}-yyyyyy+
{c1}   .oyyyyyyo. :yyyyyy/{c2}-yyyyyy+ ---------
{c1}  .syyyyyy+`  :yyyyyy/{c2}-yyyyy+-+syyyyyyyy
{c1} -syyyyyy/    :yyyyyy/{c2}-yyys:.syyyyyyyyyy
{c1}:syyyyyy/     :yyyyyy/{c2}-yyo.:syyyyyyyyyyy"##########
    )
}

// pub fn AsteroidOS() -> String {
//     let c1 = COLORS[160];
//     let c2 = COLORS[208];
//     let c3 = COLORS[202];
//     let c4 = COLORS[214];
//     format!(
//         r##########"{c1}                    ***
// {c1}                   *****
// {c1}                **********
// {c1}              ***************
// {c1}           *///****////****////.
// {c2}         (/////// /////// ///////(
// {c2}      /(((((//*     //,     //((((((.
// {c2}    (((((((((((     (((        ((((((((
// {c2} *(((((((((((((((((((((((        ((((((((
// {c3}    (((((#(((((((#(((((        ((#(((((
// {c3}     (#(#(#####(#(#,       ####(#(#
// {c3}         #########        ########
// {c3}           /########   ########
// {c4}              #######%#######
// {c4}                (#%%%%%%%#
// {c4}                   %%%%%
// {c4}                    %%%"##########
//     )
// }

pub fn bedrock() -> String {
    let c1 = COLORS[8];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}--------------------------------------
--------------------------------------
--------------------------------------
---{c2}\\\\\\\\\\\\\\\\\\\\\\\\{c1}-----------------------
----{c2}\\\\\\      \\\\\\{c1}----------------------
-----{c2}\\\\\\      \\\\\\{c1}---------------------
------{c2}\\\\\\      \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\{c1}------
-------{c2}\\\\\\                    \\\\\\{c1}-----
--------{c2}\\\\\\                    \\\\\\{c1}----
---------{c2}\\\\\\        ______      \\\\\\{c1}---
----------{c2}\\\\\\                   ///{c1}---
-----------{c2}\\\\\\                 ///{c1}----
------------{c2}\\\\\\               ///{c1}-----
-------------{c2}\\\\\\////////////////{c1}------
--------------------------------------
--------------------------------------
--------------------------------------"##########
    )
}

pub fn bitrig() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}   `hMMMMN+
   -MMo-dMd`
   oMN- oMN`
   yMd  /NM:
  .mMmyyhMMs
  :NMMMhsmMh
  +MNhNNoyMm-
  hMd.-hMNMN:
  mMmsssmMMMo
 .MMdyyhNMMMd
 oMN.`/dMddMN`
 yMm/hNm+./MM/
.dMMMmo.``.NMo
:NMMMNmmmmmMMh
/MN/-------oNN:
hMd.       .dMh
sm/         /ms"##########
    )
}

pub fn black_arch() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[1];
    let c3 = COLORS[0];
    format!(
        r##########"{c3}                   00
                   11
                  ===={c1}
                  .{c3}//{c1}
                 `o{c3}//{c1}:
                `+o{c3}//{c1}o:
               `+oo{c3}//{c1}oo:
               -+oo{c3}//{c1}oo+:
             `/:-:+{c3}//{c1}ooo+:
            `/+++++{c3}//{c1}+++++:
           `/++++++{c3}//{c1}++++++:
          `/+++o{c2}ooo{c3}//{c2}ooo{c1}oooo/`
{c2}         {c1}./{c2}ooosssso{c3}//{c2}osssssso{c1}+`
{c2}        .oossssso-`{c3}//{c1}`/ossssss+`
       -osssssso.  {c3}//{c1}  :ssssssso.
      :osssssss/   {c3}//{c1}   osssso+++.
     /ossssssss/   {c3}//{c1}   +ssssooo/-
   `/ossssso+/:-   {c3}//{c1}   -:/+osssso+-
  `+sso+:-`        {c3}//{c1}       `.-/+oso:
 `++:.             {c3}//{c1}            `-/+/
 .`                {c3}/{c1}                `/"##########
    )
}

pub fn blag() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}             d
            ,MK:
            xMMMX:
           .NMMMMMX;
           lMMMMMMMM0clodkO0KXWW:
           KMMMMMMMMMMMMMMMMMMX'
      .;d0NMMMMMMMMMMMMMMMMMMK.
 .;dONMMMMMMMMMMMMMMMMMMMMMMx
'dKMMMMMMMMMMMMMMMMMMMMMMMMl
   .:xKWMMMMMMMMMMMMMMMMMMM0.
       .:xNMMMMMMMMMMMMMMMMMK.
          lMMMMMMMMMMMMMMMMMMK.
          ,MMMMMMMMWkOXWMMMMMM0
          .NMMMMMNd.     `':ldko
           OMMMK:
           oWk,
           ;:"##########
    )
}

pub fn blank_on() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}        `./ohdNMMMMNmho+.` {c1}       .+oo:`
{c2}      -smMMMMMMMMMMMMMMMMmy-`    {c1}`yyyyy+
{c2}   `:dMMMMMMMMMMMMMMMMMMMMMMd/`  {c1}`yyyyys
{c2}  .hMMMMMMMNmhso/++symNMMMMMMMh- {c1}`yyyyys
{c2} -mMMMMMMms-`         -omMMMMMMN-{c1}.yyyyys
{c2}.mMMMMMMy.              .yMMMMMMm:{c1}yyyyys
{c2}sMMMMMMy                 `sMMMMMMh{c1}yyyyys
{c2}NMMMMMN:                  .NMMMMMN{c1}yyyyys
{c2}MMMMMMm.                   NMMMMMN{c1}yyyyys
{c2}hMMMMMM+                  /MMMMMMN{c1}yyyyys
{c2}:NMMMMMN:                :mMMMMMM+{c1}yyyyys
{c2} oMMMMMMNs-            .sNMMMMMMs.{c1}yyyyys
{c2}  +MMMMMMMNho:.`  `.:ohNMMMMMMNo {c1}`yyyyys
{c2}   -hMMMMMMMMNNNmmNNNMMMMMMMMh-  {c1}`yyyyys
{c2}     :yNMMMMMMMMMMMMMMMMMMNy:`   {c1}`yyyyys
{c2}       .:sdNMMMMMMMMMMNds/.      {c1}`yyyyyo
{c2}           `.:/++++/:.`           {c1}:oys+."##########
    )
}

pub fn blue_light() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[4];
    format!(
        r##########"{c1}              oMMNMMMMMMMMMMMMMMMMMMMMMM
              oMMMMMMMMMMMMMMMMMMMMMMMMM
              oMMMMMMMMMMMMMMMMMMMMMMMMM
              oMMMMMMMMMMMMMMMMMMMMMMMMM
              -+++++++++++++++++++++++mM{c2}
             ```````````````````````..{c1}dM{c2}
           ```````````````````````....{c1}dM{c2}
         ```````````````````````......{c1}dM{c2}
       ```````````````````````........{c1}dM{c2}
     ```````````````````````..........{c1}dM{c2}
   ```````````````````````............{c1}dM{c2}
.::::::::::::::::::::::-..............{c1}dM{c2}
 `-+yyyyyyyyyyyyyyyyyyyo............{c1}+mMM{c2}
     -+yyyyyyyyyyyyyyyyo..........{c1}+mMMMM{c2}
        ./syyyyyyyyyyyyo........{c1}+mMMMMMM{c2}
           ./oyyyyyyyyyo......{c1}+mMMMMMMMM{c2}
              omdyyyyyyo....{c1}+mMMMMMMMMMM{c2}
              {c1}oMMM{c2}mdhyyo..{c1}+mMMMMMMMMMMMM
              oNNNNNNm{c2}dso{c1}mMMMMMMMMMMMMMM"##########
    )
}

pub fn bonsai() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[2];
    let c3 = COLORS[3];
    format!(
        r##########"{c2}   ,####,
   {c2}#######,  {c2},#####,
   {c2}#####',#  {c2}'######
    {c2}''###'{c3}';,,,'{c2}###'
   {c3}       ,;  ''''
   {c3}      ;;;   {c2},#####,
   {c3}     ;;;'  ,,;{c2};;###
   {c3}     ';;;;''{c2}'####'
   {c3}      ;;;
   {c3}   ,.;;';'',,,
   {c3}  '     '
{c1} #
 #                        O
 ##, ,##,',##, ,##  ,#,   ,
 # # #  # #''# #,,  # #   #
 '#' '##' #  #  ,,# '##;, #"##########
    )
}

// pub fn BSD() -> String {
// let c1 = Colors[1];
// let c2 = Colors[7];
// let c3 = Colors[4];
// let c4 = Colors[3];
// let c5 = Colors[6];
//   format!(
//     r##########"{c1}             ,        ,
//             /(        )`
//             \ \___   / |
//             /- _  `-/  '
//            ({c2}/\/ \ {c1}\   /\
//            {c2}/ /   | `    {c1}\
//            {c3}O O   {c2}) {c1}/    |
//            {c2}`-^--'{c1}`<     '
//           (_.)  _  )   /
//            `.___/`    /
//              `-----' /
// {c4}<----.     __ / __   \
// {c4}<----|===={c1}O))){c4}=={c1}) \) /{c4}====|
// <----'    {c1}`--' `.__,' \
//              |        |
//               \       /       /\
//          {c5}______{c1}( (_  / \______/
//        {c5},'  ,-----'   |
//        `--{__________)"##########
//   )
// }

pub fn bunsen_labs() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}        `++
      -yMMs
    `yMMMMN`
   -NMMMMMMm.
  :MMMMMMMMMN-
 .NMMMMMMMMMMM/
 yMMMMMMMMMMMMM/
`MMMMMMNMMMMMMMN.
-MMMMN+ /mMMMMMMy
-MMMm`   `dMMMMMM
`MMN.     .NMMMMM.
 hMy       yMMMMM`
 -Mo       +MMMMN
  /o       +MMMMs
           +MMMN`
           hMMM:
          `NMM/
          +MN:
          mh.
         -/"##########
    )
}

pub fn calculate() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[3];
    format!(
        r##########"{c1}                              ......
                           ,,+++++++,.
                         .,,,....,,,{c2}+**+,,.{c1}
                       ............,{c2}++++,,,{c1}
                      ...............
                    ......,,,........
                  .....+*#####+,,,*+.
              .....,*###############,..,,,,,,..
           ......,*#################*..,,,,,..,,,..
         .,,....*####################+***+,,,,...,++,
       .,,..,..*#####################*,
     ,+,.+*..*#######################.
   ,+,,+*+..,########################*
.,++++++.  ..+##**###################+
.....      ..+##***#################*.
           .,.*#*****##############*.
           ..,,*********#####****+.
     {c2}.,++*****+++{c1}*****************{c2}+++++,.{c1}
      {c2},++++++**+++++{c1}***********{c2}+++++++++,{c1}
     {c2}.,,,,++++,..  .,,,,,.....,+++,.,,{c1}"##########
    )
}

pub fn carbs() -> String {
    let c2 = COLORS[5];
    format!(
        r##########"{c2}             ..........
          ..,;:ccccccc:;'..
       ..,clllc:;;;;;:cllc,.
      .,cllc,...     ..';;'.
     .;lol;..           ..
    .,lol;.
    .coo:.
   .'lol,.
   .,lol,.
   .,lol,.
    'col;.
    .:ooc'.
    .'col:.
     .'cllc'..          .''.
      ..:lolc,'.......',cll,.
        ..;cllllccccclllc;'.
          ...',;;;;;;,,...
                ....."##########
    )
}

pub fn cbl_mariner() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                    .
                  :-  .
                :==. .=:
              :===:  -==:
            :-===:  .====:
          :-====-   -=====:
         -======   :=======:
        -======.  .=========:
       -======:   -==========.
      -======-    -===========.
     :======-      :===========.
    :=======.       .-==========.
   :=======:          -==========.
  :=======-            :==========.
 :=======-              .-========-
:--------.                :========-
                    ..:::--=========-
            ..::---================-=-"##########
    )
}

pub fn cel_os() -> String {
    let c3 = COLORS[0];
    let c4 = COLORS[5];
    format!(
        r##########"{c4}                     .,cmmmmmmmmmmmc,.
                .,cmMMMMMMMMMMMMMMMMMMMMmc.
             .cMMMMMMMMMMMMMMMMMMMMMMMMMMMmc.
           .cMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMc.
         ,:MMM {c3}####################################{c4}
        cMMMMMMmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmc.
       .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM.
      .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMc
      "******************************MMMMMMMMMMMMMc:
{c3}#################################### {c4}MMMMMMMMMMMMMc
      "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:
       "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"
       'MMMMMMMMM*******************************:
        \"MMMMMM {c3}#####################################
         {c4}`:MMMMMMmmmmmmmmmmmmmmmmmmmmmmmmmmmmm;
           `"MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"
             `":MMMMMMMMMMMMMMMMMMMMMMMMM;'
                `":MMMMMMMMMMMMMMMMMMM:"
                     "************""##########
    )
}

pub fn centos_small() -> String {
    let c1 = COLORS[3];
    let c2 = COLORS[2];
    let c3 = COLORS[4];
    let c4 = COLORS[5];
    format!(
        r##########"{c2} ____{c1}^{c4}____
{c2} |\\  {c1}|{c4}  /|
{c2} | \\ {c1}|{c4} / |
{c4}<---- {c3}---->
{c3} | / {c2}|{c1} \\ |
{c3} |/__{c2}|{c1}__\\|
{c2}     v"##########
    )
}

pub fn cent_os() -> String {
    let c1 = COLORS[3];
    let c2 = COLORS[2];
    let c3 = COLORS[4];
    let c4 = COLORS[5];
    format!(
        r##########"{c1}                 ..
               .PLTJ.
              <><><><>
     {c2}KKSSV' 4KKK {c1}LJ{c4} KKKL.'VSSKK
     {c2}KKV' 4KKKKK {c1}LJ{c4} KKKKAL 'VKK
     {c2}V' ' 'VKKKK {c1}LJ{c4} KKKKV' ' 'V
     {c2}.4MA.' 'VKK {c1}LJ{c4} KKV' '.4Mb.
{c4}   . {c2}KKKKKA.' 'V {c1}LJ{c4} V' '.4KKKKK {c3}.
{c4} .4D {c2}KKKKKKKA.'' {c1}LJ{c4} ''.4KKKKKKK {c3}FA.
{c4}<QDD ++++++++++++  {c3}++++++++++++ GFD>
{c4} 'VD {c3}KKKKKKKK'.. {c2}LJ {c1}..'KKKKKKKK {c3}FV
{c4}   ' {c3}VKKKKK'. .4 {c2}LJ {c1}K. .'KKKKKV {c3}'
     {c3} 'VK'. .4KK {c2}LJ {c1}KKA. .'KV'
     {c3}A. . .4KKKK {c2}LJ {c1}KKKKA. . .4
     {c3}KKA. 'KKKKK {c2}LJ {c1}KKKKK' .4KK
     {c3}KKSSA. VKKK {c2}LJ {c1}KKKV .4SSKK
{c2}              <><><><>
               'MKKM'
                 ''"##########
    )
}

pub fn chakra() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}     _ _ _        "kkkkkkkk.
   ,kkkkkkkk.,    'kkkkkkkkk,
   ,kkkkkkkkkkkk., 'kkkkkkkkk.
  ,kkkkkkkkkkkkkkkk,'kkkkkkkk,
 ,kkkkkkkkkkkkkkkkkkk'kkkkkkk.
  "''"''',;::,,"''kkk''kkkkk;   __
      ,kkkkkkkkkk, "k''kkkkk' ,kkkk
    ,kkkkkkk' ., ' .: 'kkkk',kkkkkk
  ,kkkkkkkk'.k'   ,  ,kkkk;kkkkkkkkk
 ,kkkkkkkk';kk 'k  "'k',kkkkkkkkkkkk
.kkkkkkkkk.kkkk.'kkkkkkkkkkkkkkkkkk'
;kkkkkkkk''kkkkkk;'kkkkkkkkkkkkk''
'kkkkkkk; 'kkkkkkkk.,""''"''""
  ''kkkk;  'kkkkkkkkkk.,
     ';'    'kkkkkkkkkkkk.,
             ';kkkkkkkkkk'
               ';kkkkkk'
                  "''""##########
    )
}

pub fn chalet_os() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}             `.//+osso+/:``
         `/sdNNmhyssssydmNNdo:`
       :hNmy+-`          .-+hNNs-
     /mMh/`       `+:`       `+dMd:
   .hMd-        -sNNMNo.  /yyy  /mMs`
  -NM+       `/dMd/--omNh::dMM   `yMd`
 .NN+      .sNNs:/dMNy:/hNmo/s     yMd`
 hMs    `/hNd+-smMMMMMMd+:omNy-    `dMo
:NM.  .omMy:/hNMMMMMMMMMMNy:/hMd+`  :Md`
/Md` `sm+.omMMMMMMMMMMMMMMMMd/-sm+  .MN:
/Md`      MMMMMMMMMMMMMMMMMMMN      .MN:
:NN.      MMMMMMm....--NMMMMMN      -Mm.
`dMo      MMMMMMd      mMMMMMN      hMs
 -MN:     MMMMMMd      mMMMMMN     oMm`
  :NM:    MMMMMMd      mMMMMMN    +Mm-
   -mMy.  mmmmmmh      dmmmmmh  -hMh.
     oNNs-                    :yMm/
      .+mMdo:`            `:smMd/`
         -ohNNmhsoo++osshmNNh+.
            `./+syyhhyys+:``"##########
    )
}

pub fn chapeau() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}               .-/-.
            ////////.
          ////////{c2}y+{c1}//.
        ////////{c2}mMN{c1}/////.
      ////////{c2}mMN+{c1}////////.
    ////////////////////////.
  /////////+{c2}shhddhyo{c1}+////////.
 ////////{c2}ymMNmdhhdmNNdo{c1}///////.
///////+{c2}mMms{c1}////////{c2}hNMh{c1}///////.
///////{c2}NMm+{c1}//////////{c2}sMMh{c1}///////
//////{c2}oMMNmmmmmmmmmmmmMMm{c1}///////
//////{c2}+MMmssssssssssssss+{c1}///////
`//////{c2}yMMy{c1}////////////////////
 `//////{c2}smMNhso++oydNm{c1}////////
  `///////{c2}ohmNMMMNNdy+{c1}///////
    `//////////{c2}++{c1}//////////
       `////////////////.
           -////////-"##########
    )
}

pub fn chrom() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[1];
    let c3 = COLORS[3];
    let c4 = COLORS[4];
    let c5 = COLORS[7];
    format!(
        r##########"{c2}            .,:loool:,.
        .,coooooooooooooc,.
     .,lllllllllllllllllllll,.
    ;ccccccccccccccccccccccccc;
{c1}  '{c2}ccccccccccccccccccccccccccccc.
{c1} ,oo{c2}c::::::::okO{c5}000{c3}0OOkkkkkkkkkkk:
{c1}.ooool{c2};;;;:x{c5}K0{c4}kxxxxxk{c5}0X{c3}K0000000000.
{c1}:oooool{c2};,;O{c5}K{c4}ddddddddddd{c5}KX{c3}000000000d
{c1}lllllool{c2};l{c5}N{c4}dllllllllllld{c5}N{c3}K000000000
{c1}lllllllll{c2}o{c5}M{c4}dccccccccccco{c5}W{c3}K000000000
{c1};cllllllllX{c5}X{c4}c:::::::::c{c5}0X{c3}000000000d
{c1}.ccccllllllO{c5}Nk{c4}c;,,,;cx{c5}KK{c3}0000000000.
{c1} .cccccclllllxOO{c5}OOO{c1}Okx{c3}O0000000000;
{c1}  .:ccccccccllllllllo{c3}O0000000OOO,
{c1}    ,:ccccccccclllcd{c3}0000OOOOOOl.
{c1}      '::ccccccccc{c3}dOOOOOOOkx:.
{c1}        ..,::cccc{c3}xOOOkkko;.
{c1}            ..,:{c3}dOkxl:."##########
    )
}

pub fn cleanjaro_small() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}█████ ██████████
█████ ██████████
█████
█████
█████
████████████████
████████████████"##########
    )
}

pub fn cleanjaro() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}███████▌ ████████████████
███████▌ ████████████████
███████▌ ████████████████
███████▌
███████▌
███████▌
███████▌
███████▌
█████████████████████████
█████████████████████████
█████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"##########
    )
}

pub fn clear_os() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}             `.--::::::--.`
         .-:////////////////:-.
      `-////////////////////////-`
     -////////////////////////////-
   `//////////////-..-//////////////`
  ./////////////:      ://///////////.
 `//////:..-////:      :////-..-//////`
 ://////`    -///:.``.:///-`    ://///:
`///////:.     -////////-`    `:///////`
.//:--////:.     -////-`    `:////--://.
./:    .////:.     --`    `:////-    :/.
`//-`    .////:.        `:////-    `-//`
 :///-`    .////:.    `:////-    `-///:
 `/////-`    -///:    :///-    `-/////`
  `//////-   `///:    :///`   .//////`
   `:////:   `///:    :///`   -////:`
     .://:   `///:    :///`   -//:.
       .::   `///:    :///`   -:.
             `///:    :///`
              `...    ...`"##########
    )
}

pub fn clear_linux_os() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[3];
    let c3 = COLORS[7];
    let c4 = COLORS[6];
    format!(
        r##########"{c1}          BBB
       BBBBBBBBB
     BBBBBBBBBBBBBBB
   BBBBBBBBBBBBBBBBBBBB
   BBBBBBBBBBB         BBB
  BBBBBBBB{c2}YYYYY
{c1}  BBBBBBBB{c2}YYYYYY
{c1}  BBBBBBBB{c2}YYYYYYY
{c1}  BBBBBBBBB{c2}YYYYY{c3}W
{c4} GG{c1}BBBBBBBY{c2}YYYY{c3}WWW
{c4} GGG{c1}BBBBBBB{c2}YY{c3}WWWWWWWW
{c4} GGGGGG{c1}BBBBBB{c3}WWWWWWWW
{c4} GGGGGGGG{c1}BBBB{c3}WWWWWWWW
{c4}GGGGGGGGGGG{c1}BBB{c3}WWWWWWW
{c4}GGGGGGGGGGGGG{c1}B{c3}WWWWWW
{c4}GGGGGGGG{c3}WWWWWWWWWWW
{c4}GG{c3}WWWWWWWWWWWWWWWW
 WWWWWWWWWWWWWWWW
      WWWWWWWWWW
          WWW"##########
    )
}

pub fn clover() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[6];
    format!(
        r##########"{c1}               `omo``omo`
             `oNMMMNNMMMNo`
           `oNMMMMMMMMMMMMNo`
          oNMMMMMMMMMMMMMMMMNo
          `sNMMMMMMMMMMMMMMNs`
     `omo`  `sNMMMMMMMMMMNs`  `omo`
   `oNMMMNo`  `sNMMMMMMNs`  `oNMMMNo`
 `oNMMMMMMMNo`  `oNMMNs`  `oNMMMMMMMNo`
oNMMMMMMMMMMMNo`  `sy`  `oNMMMMMMMMMMMNo
`sNMMMMMMMMMMMMNo.{c2}oNNs{c1}.oNMMMMMMMMMMMMNs`
`oNMMMMMMMMMMMMNs.{c2}oNNs{c1}.oNMMMMMMMMMMMMNo`
oNMMMMMMMMMMMNs`  `sy`  `oNMMMMMMMMMMMNo
 `oNMMMMMMMNs`  `oNMMNo`  `oNMMMMMMMNs`
   `oNMMMNs`  `sNMMMMMMNs`  `oNMMMNs`
     `oNs`  `sNMMMMMMMMMMNs`  `oNs`
          `sNMMMMMMMMMMMMMMNs`
          +NMMMMMMMMMMMMMMMMNo
           `oNMMMMMMMMMMMMNo`
             `oNMMMNNMMMNs`
               `omo``oNs`"##########
    )
}

pub fn condres() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[3];
    let c3 = COLORS[6];
    format!(
        r##########"{c1}syyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy+{c3}.+.
{c1}`oyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy+{c3}:++.
{c2}/o{c1}+oyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy/{c3}oo++.
{c2}/y+{c1}syyyyyyyyyyyyyyyyyyyyyyyyyyyyy{c3}+ooo++.
{c2}/hy+{c1}oyyyhhhhhhhhhhhhhhyyyyyyyyy{c3}+oo+++++.
{c2}/hhh+{c1}shhhhhdddddhhhhhhhyyyyyyy{c3}+oo++++++.
{c2}/hhdd+{c1}oddddddddddddhhhhhyyyys{c3}+oo+++++++.
{c2}/hhddd+{c1}odmmmdddddddhhhhyyyy{c3}+ooo++++++++.
{c2}/hhdddmo{c1}odmmmdddddhhhhhyyy{c3}+oooo++++++++.
{c2}/hdddmmms{c1}/dmdddddhhhhyyys{c3}+oooo+++++++++.
{c2}/hddddmmmy{c1}/hdddhhhhyyyyo{c3}+oooo++++++++++:
{c2}/hhdddmmmmy{c1}:yhhhhyyyyy+{c3}+oooo+++++++++++:
{c2}/hhddddddddy{c1}-syyyyyys+{c3}ooooo++++++++++++:
{c2}/hhhddddddddy{c1}-+yyyy+{c3}/ooooo+++++++++++++:
{c2}/hhhhhdddddhhy{c1}./yo:{c3}+oooooo+++++++++++++/
{c2}/hhhhhhhhhhhhhy{c1}:-.{c3}+sooooo+++++++++++///:
{c2}:sssssssssssso++{c1}{c3}`:/:--------.````````"##########
    )
}

pub fn container_linux_by_core_os() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    let c3 = COLORS[1];
    format!(
        r##########"{c1}                .....
          .';:cccccccc:;'.
        ':ccccclc{c3}lllllllll{c1}cc:.
     .;cccccccc{c3}lllllllllllllll{c1}c,
    ;clllccccc{c3}llllllllllllllllll{c1}c,
  .cllclccccc{c3}lllll{c2}lll{c3}llllllllllll{c1}c:
  ccclclcccc{c3}cllll{c2}kWMMNKk{c3}llllllllll{c1}c:
 :ccclclcccc{c3}llll{c2}oWMMMMMMWO{c3}lllllllll{c1}c,
.ccllllllccc{c3}clll{c2}OMMMMMMMMM0{c3}lllllllll{c1}c
.lllllclcccc{c3}llll{c2}KMMMMMMMMMMo{c3}llllllll{c1}c.
.lllllllcccc{c3}clll{c2}KMMMMMMMMN0{c3}lllllllll{c1}c.
.cclllllcccc{c3}lllld{c2}xkkxxdo{c3}llllllllllc{c1}lc
 :cccllllllcccc{c3}lllccllllcclccc{c1}cccccc;
 .ccclllllllcccccccc{c3}lll{c1}ccccclccccccc
  .cllllllllllclcccclccclccllllcllc
    :cllllllllccclcllllllllllllcc;
     .cccccccccccccclcccccccccc:.
       .;cccclccccccllllllccc,.
          .';ccccclllccc:;..
                ....."##########
    )
}

pub fn crux_small() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[5];
    let c3 = COLORS[7];
    format!(
        r##########"{c1}    ___
   ({c3}.· {c1}|
   ({c2}<> {c1}|
  / {c3}__  {c1}\\
 ( {c3}/  \\ {c1}/|
{c2}_{c1}/\\ {c3}__){c1}/{c2}_{c1})
{c2}\/{c1}-____{c2}\/"##########
    )
}

pub fn crux() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[5];
    let c3 = COLORS[7];
    format!(
        r##########"{c1}         odddd
      oddxkkkxxdoo
     ddcoddxxxdoool
     xdclodod  olol
     xoc  xdd  olol
     xdc  {c2}k00{c1}Okdlol
     xxd{c2}kOKKKOkd{c1}ldd
     xdco{c2}xOkdlo{c1}dldd
     ddc:cl{c2}lll{c1}oooodo
   odxxdd{c3}xkO000kx{c1}ooxdo
  oxdd{c3}x0NMMMMMMWW0od{c1}kkxo
 oooxd{c3}0WMMMMMMMMMW0o{c1}dxkx
docldkXW{c3}MMMMMMMWWN{c1}Odolco
xx{c2}dx{c1}kxxOKN{c3}WMMWN{c1}0xdoxo::c
{c2}xOkkO{c1}0oo{c3}odOW{c2}WW{c1}XkdodOxc:l
{c2}dkkkxkkk{c3}OKX{c2}NNNX0Oxx{c1}xc:cd
{c2} odxxdx{c3}xllod{c2}ddooxx{c1}dc:ldo
{c2}   lodd{c1}dolccc{c2}ccox{c1}xoloo"##########
    )
}

pub fn crystal_linux() -> String {
    let c1 = COLORS[13];
    format!(
        r##########"{c1}                        mysssym
{c1}                      mysssym
{c1}                    mysssym
{c1}                  mysssym
{c1}                mysssyd
{c1}              mysssyd    N
{c1}            mysssyd    mysym
{c1}          mysssyd      dysssym
{c1}        mysssyd          dysssym
{c1}      mysssyd              dysssym
{c1}      mysssyd              dysssym
{c1}        mysssyd          dysssym
{c1}          mysssyd      dysssym
{c1}            mysym    dysssym
{c1}              N    dysssym
{c1}                 dysssym
{c1}               dysssym
{c1}             dysssym
{c1}           dysssym
{c1}         dysssym"##########
    )
}

pub fn cucumber() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[3];
    format!(
        r##########"{c1}           `.-://++++++//:-.`
        `:/+//{c2}::--------{c1}:://+/:`
      -++/:{c2}----..........----{c1}:/++-
    .++:{c2}---...........-......---{c1}:++.
   /+:{c2}---....-::/:/--//:::-....---{c1}:+/
 `++:{c2}--.....:---::/--/::---:.....--{c1}:++`
 /+:{c2}--.....--.--::::-/::--.--.....--{c1}:+/
-o:{c2}--.......-:::://--/:::::-.......--{c1}:o-
/+:{c2}--...-:-::---:::..:::---:--:-...--{c1}:+/
o/:{c2}-...-:.:.-/:::......::/:.--.:-...-{c1}:/o
o/{c2}--...::-:/::/:-......-::::::-/-...-{c1}:/o
/+:{c2}--..-/:/:::--:::..:::--::////-..--{c1}:+/
-o:{c2}--...----::/:::/--/:::::-----...--{c1}:o-
 /+:{c2}--....://:::.:/--/:.::://:....--{c1}:+/
 `++:{c2}--...-:::.--.:..:.--.:/:-...--{c1}:++`
   /+:{c2}---....----:-..-:----....---{c1}:+/
    .++:{c2}---..................---{c1}:++.
      -/+/:{c2}----..........----{c1}:/+/-
        `:/+//{c2}::--------:::{c1}/+/:`
           `.-://++++++//:-.`"##########
    )
}

pub fn dahlia() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}
                  .#.
                *%@@@%*
        .,,,,,(&@@@@@@@&/,,,,,.
       ,#@@@@@@@@@@@@@@@@@@@@@#.
       ,#@@@@@@@&#///#&@@@@@@@#.
     ,/%&@@@@@%/,    .,(%@@@@@&#/.
   *#&@@@@@@#,.         .*#@@@@@@&#,
 .&@@@@@@@@@(            .(@@@@@@@@@&&.
#@@@@@@@@@@(               )@@@@@@@@@@@#
 °@@@@@@@@@@(            .(@@@@@@@@@@@°
   *%@@@@@@@(.           ,#@@@@@@@%*
     ,(&@@@@@@%*.     ./%@@@@@@%(,
       ,#@@@@@@@&(***(&@@@@@@@#.
       ,#@@@@@@@@@@@@@@@@@@@@@#.
        ,*****#&@@@@@@@&(*****,
               ,/%@@@%/.
                  ,#,"##########
    )
}

pub fn debian_small() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}  _____
 /  __ \\
|  /    |
|  \\___-
-_
  --_"##########
    )
}

pub fn debian() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}       _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P"        """Y$$.".
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P"'   {c1}.{c2}    $$$
 $$P      d$'     {c1},{c2}    $$P
 $$:      $$.   {c1}-{c2}    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    {c1}`.{c2}`"Y$$$$P"'
{c2} `$$b      {c1}"-.__
{c2}  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `"Y$b._
              `""""##########
    )
}

pub fn deepin() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}             ............
         .';;;;;.       .,;,.
      .,;;;;;;;.       ';;;;;;;.
    .;::::::::'     .,::;;,''''',.
   ,'.::::::::    .;;'.          ';
  ;'  'cccccc,   ,' :: '..        .:
 ,,    :ccccc.  ;: .c, '' :.       ,;
.l.     cllll' ., .lc  :; .l'       l.
.c       :lllc  ;cl:  .l' .ll.      :'
.l        'looc. .   ,o:  'oo'      c,
.o.         .:ool::coc'  .ooo'      o.
 ::            .....   .;dddo      ;c
  l:...            .';lddddo.     ,o
   lxxxxxdoolllodxxxxxxxxxc      :l
    ,dxxxxxxxxxxxxxxxxxxl.     'o,
      ,dkkkkkkkkkkkkko;.    .;o;
        .;okkkkkdl;.    .,cl:.
            .,:cccccccc:,."##########
    )
}

pub fn desa_os() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}███████████████████████
███████████████████████
███████████████████████
███████████████████████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
████████               ███████
██████████████████████████████
██████████████████████████████
████████████████████████
████████████████████████
████████████████████████"##########
    )
}

pub fn devuan() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}   ..,,;;;::;,..
           `':ddd;:,.
                 `'dPPd:,.
                     `:b$$b`.
                        'P$$$d`
                         .$$$$$`
                         ;$$$$$P
                      .:P$$$$$$`
                  .,:b$$$$$$$;'
             .,:dP$$$$$$$$b:'
      .,:;db$$$$$$$$$$Pd'`
 ,db$$$$$$$$$$$$$$b:'`
:$$$$$$$$$$$$b:'`
 `$$$$$bd:''`
   `'''`"##########
    )
}

pub fn drac_os() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}       `-:/-
          -os:
            -os/`
              :sy+-`
               `/yyyy+.
                 `+yyyyo-
                   `/yyyys:
`:osssoooo++-        +yyyyyy/`
   ./yyyyyyo         yo`:syyyy+.
      -oyyy+         +-   :yyyyyo-
        `:sy:        `.    `/yyyyys:
           ./o/.`           .oyyso+oo:`
              :+oo+//::::///:-.`     `.`"##########
    )
}

// pub fn DarkOs() -> String {
// let c1 = Colors[1];
// let c2 = Colors[6];
// let c3 = Colors[5];
// let c4 = Colors[3];
// let c5 = Colors[2];
//   format!(
//     r##########"{c3}⠀⠀⠀⠀  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠢⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c1}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣶⠋⡆⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c5}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡆⢀⣤⢛⠛⣠⣿⠀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c6}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣶⣿⠟⣡⠊⣠⣾⣿⠃⣠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c2}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣯⣿⠀⠊⣤⣿⣿⣿⠃⣴⣧⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c1}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣶⣿⣿⡟⣠⣶⣿⣿⣿⢋⣤⠿⠛⠉⢁⣭⣽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c4}  ⠀⠀⠀⠀⠀⠀ ⠀⣠⠖⡭⢉⣿⣯⣿⣯⣿⣿⣿⣟⣧⠛⢉⣤⣶⣾⣿⣿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c5}⠀⠀⠀⠀⠀⠀⠀⠀⣴⣫⠓⢱⣯⣿⢿⠋⠛⢛⠟⠯⠶⢟⣿⣯⣿⣿⣿⣿⣿⣿⣦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c2}⠀⠀⠀⠀⠀⠀⢀⡮⢁⣴⣿⣿⣿⠖⣠⠐⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠛⠛⠛⢿⣶⣄⠀⠀⠀⠀⠀⠀⠀
// {c3}⠀⠀⠀⠀⢀⣤⣷⣿⣿⠿⢛⣭⠒⠉⠀⠀⠀⣀⣀⣄⣤⣤⣴⣶⣶⣶⣿⣿⣿⣿⣿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀
// {c1}⠀⢀⣶⠏⠟⠝⠉⢀⣤⣿⣿⣶⣾⣿⣿⣿⣿⣿⣿⣟⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c6}⢴⣯⣤⣶⣿⣿⣿⣿⣿⡿⣿⣯⠉⠉⠉⠉⠀⠀⠀⠈⣿⡀⣟⣿⣿⢿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c5}⠀⠀⠀⠉⠛⣿⣧⠀⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠃⣿⣿⣯⣿⣦⡀⠀⠉⠻⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c3}⠀⠀⠀⠀⠀⠀⠉⢿⣮⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠀⣯⠉⠉⠛⢿⣿⣷⣄⠀⠈⢻⣆⠀⠀⠀⠀⠀⠀⠀⠀
// {c2}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⠀⠀⠀⠀⠀⠀⠀⢀⢡⠃⣾⣿⣿⣦⠀⠀⠀⠙⢿⣿⣤⠀⠙⣄⠀⠀⠀⠀⠀⠀⠀
// {c6}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢋⡟⢠⣿⣿⣿⠋⢿⣄⠀⠀⠀⠈⡄⠙⣶⣈⡄⠀⠀⠀⠀⠀⠀
// {c1}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠚⢲⣿⠀⣾⣿⣿⠁⠀⠀⠉⢷⡀⠀⠀⣇⠀⠀⠈⠻⡀⠀⠀⠀⠀⠀
// {c4}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢢⣀⣿⡏⠀⣿⡿⠀⠀⠀⠀⠀⠀⠙⣦⠀⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c3}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠿⣧⣾⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣮⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// {c5}⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"##########
//   )
// }

pub fn itc() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}....................-==============+...
{c1}....................-==============:...
{c1}...:===========-....-==============:...
{c1}...-===========:....-==============-...
{c1}....*==========+........-::********-...
{c1}....*===========+.:*====**==*+-.-......
{c1}....:============*+-..--:+**====*---...
{c1}......::--........................::...
{c1}..+-:+-.+::*:+::+:-++::++-.:-.*.:++:++.
{c1}..:-:-++++:-::--:+::-::.:++-++:++--:-:.    ⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"##########
    )
}

pub fn dragonfly_old() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    let c3 = COLORS[3];
    format!(
        r##########"{c1}                   .-.
                 {c3} (){c1}I{c3}()
            {c1} "==.__:-:__.=="
            "==.__/~|~\__.=="
            "==._(  Y  )_.=="
 {c2}.-'~~""~=--...,__{c1}\/|\/{c2}__,...--=~""~~'-.
(               ..={c1}\\={c1}/{c2}=..               )
 `'-.        ,.-"`;{c1}/=\\{c2};"-.,_        .-'`
     `~"-=-~` .-~` {c1}|=|{c2} `~-. `~-=-"~`
          .-~`    /{c1}|=|{c2}\    `~-.
       .~`       / {c1}|=|{c2} \       `~.
   .-~`        .'  {c1}|=|{c2}  `.        `~-.
 (`     _,.-="`  {c1}  |=|{c2}    `"=-.,_     `)
  `~"~"`        {c1}   |=|{c2}           `"~"~`
                 {c1}  /=\\
                   \\=/
                    ^"##########
    )
}

pub fn dragonfly_small() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}   ,{c1}_{c2},
('-_{c1}|{c2}_-')
 >--{c1}|{c2}--<
(_-'{c1}|{c2}'-_)
    {c1}|
    |
    |"##########
    )
}

pub fn dragon_fly() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    let c3 = COLORS[3];
    format!(
        r##########"{c2},--,           {c1}|           {c2},--,
{c2}|   `-,       {c1},^,       {c2},-'   |
{c2} `,    `-,   {c3}(/ \)   {c2},-'    ,'
{c2}   `-,    `-,{c1}/   \{c2},-'    ,-'
{c2}      `------{c1}(   ){c2}------'
{c2}  ,----------{c1}(   ){c2}----------,
{c2} |        _,-{c1}(   ){c2}-,_        |
{c2}  `-,__,-'   {c1}\   /{c2}   `-,__,-'
{c1}              | |
              | |
              | |
              | |
              | |
              | |
              `|'"##########
    )
}

pub fn drauger() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}                  -``-
                `:+``+:`
               `/++``++/.
              .++/.  ./++.
             :++/`    `/++:
           `/++:        :++/`
          ./+/-          -/+/.
         -++/.            ./++-
        :++:`              `:++:
      `/++-                  -++/`
     ./++.                    ./+/.
    -++/`                      `/++-
   :++:`                        `:++:
 `/++-                            -++/`
.:-.`..............................`.-:.
`.-/++++++++++++++++++++++++++++++++/-.`"##########
    )
}

pub fn elementary_small() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}  _______
 / ____  \\
/  |  /  /\\
|__\\ /  / |
\\   /__/  /
 \\_______/"##########
    )
}

pub fn elementary() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}         eeeeeeeeeeeeeeeee
      eeeeeeeeeeeeeeeeeeeeeee
    eeeee  eeeeeeeeeeee   eeeee
  eeee   eeeee       eee     eeee
 eeee   eeee          eee     eeee
eee    eee            eee       eee
eee   eee            eee        eee
ee    eee           eeee       eeee
ee    eee         eeeee      eeeeee
ee    eee       eeeee      eeeee ee
eee   eeee   eeeeee      eeeee  eee
eee    eeeeeeeeee     eeeeee    eee
 eeeeeeeeeeeeeeeeeeeeeeee    eeeee
  eeeeeeee eeeeeeeeeeee      eeee
    eeeee                 eeeee
      eeeeeee         eeeeeee
         eeeeeeeeeeeeeeeee"##########
    )
}

pub fn endeavour_os() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[5];
    let c3 = COLORS[4];
    format!(
        r##########"{c1}                     ./{c2}o{c3}.
{c1}                   ./{c2}sssso{c3}-
{c1}                 `:{c2}osssssss+{c3}-
{c1}               `:+{c2}sssssssssso{c3}/.
{c1}             `-/o{c2}ssssssssssssso{c3}/.
{c1}           `-/+{c2}sssssssssssssssso{c3}+:`
{c1}         `-:/+{c2}sssssssssssssssssso{c3}+/.
{c1}       `.://o{c2}sssssssssssssssssssso{c3}++-
{c1}      .://+{c2}ssssssssssssssssssssssso{c3}++:
{c1}    .:///o{c2}ssssssssssssssssssssssssso{c3}++:
{c1}  `:////{c2}ssssssssssssssssssssssssssso{c3}+++.
{c1}`-////+{c2}ssssssssssssssssssssssssssso{c3}++++-
{c1} `..-+{c2}oosssssssssssssssssssssssso{c3}+++++/`
   ./++++++++++++++++++++++++++++++/:.
  `:::::::::::::::::::::::::------``"##########
    )
}

pub fn endless() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}           `:+yhmNMMMMNmhy+:`
        -odMMNhso//////oshNMMdo-
      /dMMh+.              .+hMMd/
    /mMNo`                    `oNMm:
  `yMMo`                        `oMMy`
 `dMN-                            -NMd`
 hMN.                              .NMh
/MM/                  -os`          /MM/
dMm    `smNmmhs/- `:sNMd+   ``       mMd
MMy    oMd--:+yMMMMMNo.:ohmMMMNy`    yMM
MMy    -NNyyhmMNh+oNMMMMMy:.  dMo    yMM
dMm     `/++/-``/yNNh+/sdNMNddMm-    mMd
/MM/          `dNy:       `-::-     /MM/
 hMN.                              .NMh
 `dMN-                            -NMd`
  `yMMo`                        `oMMy`
    /mMNo`                    `oNMm/
      /dMMh+.              .+hMMd/
        -odMMNhso//////oshNMMdo-
           `:+yhmNMMMMNmhy+:`"##########
    )
}

pub fn euro_linux() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}                __
         -wwwWWWWWWWWWwww-
        -WWWWWWWWWWWWWWWWWWw-
          \WWWWWWWWWWWWWWWWWWW-
  _Ww      `WWWWWWWWWWWWWWWWWWWw
 -W{c2}E{c1}Www                -WWWWWWWWW-
_WW{c2}U{c1}WWWW-                _WWWWWWWW
_WW{c2}R{c1}WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW-
wWW{c2}O{c1}WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWW{c2}L{c1}WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWw
WWW{c2}I{c1}WWWWWWWWWWWWWWWWWWWWWWWWWWWWww-
wWW{c2}N{c1}WWWWw
 WW{c2}U{c1}WWWWWWw
 wW{c2}X{c1}WWWWWWWWww
   wWWWWWWWWWWWWWWWw
    wWWWWWWWWWWWWWWWw
       WWWWWWWWWWWWWw
           wWWWWWWWw"##########
    )
}

pub fn exherbo() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    let c3 = COLORS[1];
    format!(
        r##########"{c2} ,
OXo.
NXdX0:    .cok0KXNNXXK0ko:.
KX  '0XdKMMK;.xMMMk, .0MMMMMXx;  ...
'NO..xWkMMx   kMMM    cMMMMMX,NMWOxOXd.
  cNMk  NK    .oXM.   OMMMMO. 0MMNo  kW.
  lMc   o:       .,   .oKNk;   ;NMMWlxW'
 ;Mc    ..   .,,'    .0M{c1}g;{c2}WMN'dWMMMMMMO
 XX        ,WMMMMW.  cM{c1}cfli{c2}WMKlo.   .kMk
.Mo        .WM{c1}GD{c2}MW.   XM{c1}WO0{c2}MMk        oMl
,M:         ,XMMWx::,''oOK0x;          NM.
'Ml      ,kNKOxxxxxkkO0XXKOd:.         oMk
 NK    .0Nxc{c3}:::::::::::::::{c2}fkKNk,      .MW
 ,Mo  .NXc{c3}::{c2}qXWXb{c3}::::::::::{c2}oo{c3}::{c2}lNK.    .MW
  ;Wo oMd{c3}:::{c2}oNMNP{c3}::::::::{c2}oWMMMx{c3}:{c2}c0M;   lMO
   'NO;W0c{c3}:::::::::::::::{c2}dMMMMO{c3}::{c2}lMk  .WM'
     xWONXdc{c3}::::::::::::::{c2}oOOo{c3}::{c2}lXN. ,WMd
      'KWWNXXK0Okxxo,{c3}:::::::{c2},lkKNo  xMMO
        :XMNxl,';:lodxkOO000Oxc. .oWMMo
          'dXMMXkl;,.        .,o0MMNo'
             ':d0XWMMMMWNNNNMMMNOl'
                   ':okKXWNKkl'"##########
    )
}

pub fn fedora_small() -> String {
    let c1 = COLORS[12];
    format!(
        r##########"{c1}        ,'''''.
       |   ,.  |
       |  |  '_'
  ,....|  |..
.'  ,_;|   ..'
|  |   |  |
|  ',_,'  |
 '.     ,'
   '''''"##########
    )
}

pub fn fedora_old() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}          /:-------------:\\
       :-------------------::
     :-----------{c2}/shhOHbmp{c1}---:\\
   /-----------{c2}omMMMNNNMMD  {c1}---:
  :-----------{c2}sMMMMNMNMP{c1}.    ---:
 :-----------{c2}:MMMdP{c1}-------    ---\\
,------------{c2}:MMMd{c1}--------    ---:
:------------{c2}:MMMd{c1}-------    .---:
:----    {c2}oNMMMMMMMMMNho{c1}     .----:
:--     .{c2}+shhhMMMmhhy++{c1}   .------/
:-    -------{c2}:MMMd{c1}--------------:
:-   --------{c2}/MMMd{c1}-------------;
:-    ------{c2}/hMMMy{c1}------------:
:--{c2} :dMNdhhdNMMNo{c1}------------;
:---{c2}:sdNMMMMNds:{c1}------------:
:------{c2}:://:{c1}-------------::
:---------------------://"##########
    )
}

pub fn fedora() -> String {
    let c1 = COLORS[12];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;{c2}.:dddl:.{c1};ccccccc;.
 .:ccccccccccccc;{c2}OWMKOOXMWd{c1};ccccccc:.
.:ccccccccccccc;{c2}KMMc{c1};cc;{c2}xMMc{c1};ccccccc:.
,cccccccccccccc;{c2}MMM.{c1};cc;{c2};WW:{c1};cccccccc,
:cccccccccccccc;{c2}MMM.{c1};cccccccccccccccc:
:ccccccc;{c2}oxOOOo{c1};{c2}MMM0OOk.{c1};cccccccccccc:
cccccc;{c2}0MMKxdd:{c1};{c2}MMMkddc.{c1};cccccccccccc;
ccccc;{c2}XM0'{c1};cccc;{c2}MMM.{c1};cccccccccccccccc'
ccccc;{c2}MMo{c1};ccccc;{c2}MMW.{c1};ccccccccccccccc;
ccccc;{c2}0MNc.{c1}ccc{c2}.xMMd{c1};ccccccccccccccc;
cccccc;{c2}dNMWXXXWM0:{c1};cccccccccccccc:,
cccccccc;{c2}.:odl:.{c1};cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccccc:;,..
  '::cccccccccccccc::;,."##########
    )
}

pub fn feren() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1} `----------`
 :+ooooooooo+.
-o+oooooooooo+-
..`/+++++++++++/...`````````````````
   .++++++++++++++++++++++++++/////-
    ++++++++++++++++++++++++++++++++//:`
    -++++++++++++++++++++++++++++++/-`
     ++++++++++++++++++++++++++++:.
     -++++++++++++++++++++++++/.
      +++++++++++++++++++++/-`
      -++++++++++++++++++//-`
        .:+++++++++++++//////-
           .:++++++++//////////-
             `-++++++---:::://///.
           `.:///+++.             `
          `........."##########
    )
}

pub fn freebsd_small() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}/\\,-'''''-,/\\
\\_)       (_/
|           |
|           |
 ;         ;
  '-_____-'"##########
    )
}

pub fn free_mi_nt() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}          ##
          ##         #########
                    ####      ##
            ####  ####        ##
####        ####  ##        ##
        ####    ####      ##  ##
        ####  ####  ##  ##  ##
            ####  ######
        ######  ##  ##  ####
      ####    ################
    ####        ##  ####
    ##            ####  ######
    ##      ##    ####  ####
    ##    ##  ##    ##  ##  ####
      ####  ##          ##  ##"##########
    )
}

pub fn frugalware() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}          `++/::-.`
         /o+++++++++/::-.`
        `o+++++++++++++++o++/::-.`
        /+++++++++++++++++++++++oo++/:-.``
       .o+ooooooooooooooooooosssssssso++oo++/:-`
       ++osoooooooooooosssssssssssssyyo+++++++o:
      -o+ssoooooooooooosssssssssssssyyo+++++++s`
      o++ssoooooo++++++++++++++sssyyyyo++++++o:
     :o++ssoooooo{c2}/-------------{c1}+syyyyyo+++++oo
    `o+++ssoooooo{c2}/-----{c1}+++++ooosyyyyyyo++++os:
    /o+++ssoooooo{c2}/-----{c1}ooooooosyyyyyyyo+oooss
   .o++++ssooooos{c2}/------------{c1}syyyyyyhsosssy-
   ++++++ssooooss{c2}/-----{c1}+++++ooyyhhhhhdssssso
  -s+++++syssssss{c2}/-----{c1}yyhhhhhhhhhhhddssssy.
  sooooooyhyyyyyh{c2}/-----{c1}hhhhhhhhhhhddddyssy+
 :yooooooyhyyyhhhyyyyyyhhhhhhhhhhdddddyssy`
 yoooooooyhyyhhhhhhhhhhhhhhhhhhhddddddysy/
-ysooooooydhhhhhhhhhhhddddddddddddddddssy
 .-:/+osssyyyysyyyyyyyyyyyyyyyyyyyyyyssy:
       ``.-/+oosysssssssssssssssssssssss
               ``.:/+osyysssssssssssssh.
                        `-:/+osyyssssyo
                                .-:+++`"##########
    )
}

pub fn funtoo() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}   .dKXXd                         .
  :XXl;:.                      .OXo
.'OXO''  .''''''''''''''''''''':XNd..'oco.lco,
xXXXXXX, cXXXNNNXXXXNNXXXXXXXXNNNNKOOK; d0O .k
  kXX  xXo  KNNN0  KNN.       'xXNo   :c; 'cc.
  kXX  xNo  KNNN0  KNN. :xxxx. 'NNo
  kXX  xNo  loooc  KNN. oNNNN. 'NNo
  kXX  xN0:.       KNN' oNNNX' ,XNk
  kXX  xNNXNNNNNNNNXNNNNNNNNXNNOxXNX0Xl
  ...  ......................... .;cc;."##########
    )
}

pub fn gallium_os() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}sooooooooooooooooooooooooooooooooooooo+:
yyooooooooooooooooooooooooooooooooo+/:::
yyysoooooooooooooooooooooooooooo+/::::::
yyyyyoooooooooooooooooooooooo+/:::::::::
yyyyyysoooooooooooooooooo++/::::::::::::
yyyyyyysoooooooooooooo++/:::::::::::::::
yyyyyyyyysoooooo{c2}sydddys{c1}+/:::::::::::::::
yyyyyyyyyysooo{c2}smMMMMMMMNd{c1}+::::::::::::::
yyyyyyyyyyyyo{c2}sMMMMMMMMMMMN{c1}/:::::::::::::
yyyyyyyyyyyyy{c2}dMMMMMMMMMMMM{c1}o//:::::::::::
yyyyyyyyyyyyy{c2}hMMMMMMMMMMMm{c1}--//::::::::::
yyyyyyyyyyyyyy{c2}hmMMMMMMMNy{c1}:..-://::::::::
yyyyyyyyyyyyyyy{c2}yyhhyys+:{c1}......://:::::::
yyyyyyyyyyyyyyys+:--...........-///:::::
yyyyyyyyyyyys+:--................://::::
yyyyyyyyyo+:-.....................-//:::
yyyyyyo+:-..........................://:
yyyo+:-..............................-//
o/:-...................................:"##########
    )
}

pub fn garuda() -> String {
    let c3 = COLORS[3];
    let c4 = COLORS[7];
    let c5 = COLORS[2];
    let c6 = COLORS[4];
    format!(
        r##########"{c3}
                     .%;888:8898898:
                   x;XxXB%89b8:b8%b88:
                .8Xxd                8X:.
              .8Xx;                    8x:.
            .tt8x          {c6}.d{c3}            x88;
         .@8x8;          {c6}.db:{c3}              xx@;
       {c4},tSXX°          .bbbbbbbbbbbbbbbbbbbB8x@;
     .SXxx            bBBBBBBBBBBBBBBBBBBBbSBX8;
   ,888S                                     pd!
  8X88/                                       q
  GBB.
   {c5}x%88        d888@8@X@X@X88X@@XX@@X@8@X.
     dxXd    dB8b8b8B8B08bB88b998888b88x.
      dxx8o                      .@@;.
        dx88                   .t@x.
          d:SS@8ba89aa67a853Sxxad.
            .d988999889889899dd."##########
    )
}

pub fn gentoo_small() -> String {
    let c1 = COLORS[5];
    let c2 = COLORS[7];
    format!(
        r##########"{c1} _-----_
(       \\
\    0   \\
{c2} \        )
 /      _/
(     _-
\____-"##########
    )
}

pub fn gentoo() -> String {
    let c1 = COLORS[5];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}         -/oyddmdhs+:.
     -o{c2}dNMMMMMMMMNNmhy+{c1}-`
   -y{c2}NMMMMMMMMMMMNNNmmdhy{c1}+-
 `o{c2}mMMMMMMMMMMMMNmdmmmmddhhy{c1}/`
 om{c2}MMMMMMMMMMMN{c1}hhyyyo{c2}hmdddhhhd{c1}o`
.y{c2}dMMMMMMMMMMd{c1}hs++so/s{c2}mdddhhhhdm{c1}+`
 oy{c2}hdmNMMMMMMMN{c1}dyooy{c2}dmddddhhhhyhN{c1}d.
  :o{c2}yhhdNNMMMMMMMNNNmmdddhhhhhyym{c1}Mh
    .:{c2}+sydNMMMMMNNNmmmdddhhhhhhmM{c1}my
       /m{c2}MMMMMMNNNmmmdddhhhhhmMNh{c1}s:
    `o{c2}NMMMMMMMNNNmmmddddhhdmMNhs{c1}+`
  `s{c2}NMMMMMMMMNNNmmmdddddmNMmhs{c1}/.
 /N{c2}MMMMMMMMNNNNmmmdddmNMNdso{c1}:`
+M{c2}MMMMMMNNNNNmmmmdmNMNdso{c1}/-
yM{c2}MNNNNNNNmmmmmNNMmhs+/{c1}-`
/h{c2}MMNNNNNNNNMNdhs++/{c1}-`
`/{c2}ohdmmddhys+++/:{c1}.`
  `-//////:--."##########
    )
}

pub fn pentoo() -> String {
    let c1 = COLORS[5];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}           `:oydNNMMMMNNdyo:`
        :yNMMMMMMMMMMMMMMMMNy:
      :dMMMMMMMMMMMMMMMMMMMMMMd:
     oMMMMMMMho/-....-/ohMMMMMMMo
    oMMMMMMy.            .yMMMMMMo
   .MMMMMMo                oMMMMMM.
   +MMMMMm                  mMMMMM+
   oMMMMMh                  hMMMMMo
 //hMMMMMm//{c1}`{c2}          {c1}`{c2}////mMMMMMh//
MMMMMMMMMMM{c1}/{c2}      {c1}/o/`{c2}  {c1}.{c2}smMMMMMMMMMMM
MMMMMMMMMMm      {c1}`NMN:{c2}    {c1}.{c2}yMMMMMMMMMM
MMMMMMMMMMMh{c1}:.{c2}              dMMMMMMMMM
MMMMMMMMMMMMMy{c1}.{c2}            {c1}-{c2}NMMMMMMMMM
MMMMMMMMMMMd:{c1}`{c2}           {c1}-{c2}yNMMMMMMMMMM
MMMMMMMMMMh{c1}`{c2}          {c1}./{c2}hNMMMMMMMMMMMM
MMMMMMMMMM{c1}s{c2}        {c1}.:{c2}ymMMMMMMMMMMMMMMM
MMMMMMMMMMN{c1}s:..-/{c2}ohNMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"##########
    )
}

pub fn glaucus() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}             ,,        ,d88P
           ,d8P    ,ad8888*
         ,888P    d88888*     ,,ad8888P*
    d   d888P   a88888P*  ,ad8888888*
  .d8  d8888:  d888888* ,d888888P*
 .888; 88888b d8888888b8888888P
 d8888J888888a88888888888888P*    ,d
 88888888888888888888888888P   ,,d8*
 888888888888888888888888888888888*
 *8888888888888888888888888888888*
  Y888888888P* `*``*888888888888*
   *^888^*            *Y888P**"##########
    )
}

pub fn g_new_sense() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                     ..,,,,..
               .oocchhhhhhhhhhccoo.
        .ochhlllllllc hhhhhh ollllllhhco.
    ochlllllllllll hhhllllllhhh lllllllllllhco
 .cllllllllllllll hlllllo  +hllh llllllllllllllc.
ollllllllllhco''  hlllllo  +hllh  ``ochllllllllllo
hllllllllc'       hllllllllllllh       `cllllllllh
ollllllh          +llllllllllll+          hllllllo
 `cllllh.           ohllllllho           .hllllc'
    ochllc.            ++++            .cllhco
       `+occooo+.                .+ooocco+'
              `+oo++++      ++++oo+'"##########
    )
}

pub fn gnome() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                               ,@@@@@@@@,
                 @@@@@@      @@@@@@@@@@@@
        ,@@.    @@@@@@@    *@@@@@@@@@@@@
       @@@@@%   @@@@@@(    @@@@@@@@@@@&
       @@@@@@    @@@@*     @@@@@@@@@#
@@@@*   @@@@,              *@@@@@%
@@@@@.
 @@@@#         @@@@@@@@@@@@@@@@
         ,@@@@@@@@@@@@@@@@@@@@@@@,
      ,@@@@@@@@@@@@@@@@@@@@@@@@@@&
    .@@@@@@@@@@@@@@@@@@@@@@@@@@@@
    @@@@@@@@@@@@@@@@@@@@@@@@@@@
   @@@@@@@@@@@@@@@@@@@@@@@@(
   @@@@@@@@@@@@@@@@@@@@%
    @@@@@@@@@@@@@@@@
     @@@@@@@@@@@@*        @@@@@@@@/
      &@@@@@@@@@@        @@@@@@@@@*
        @@@@@@@@@@@,    @@@@@@@@@*
          ,@@@@@@@@@@@@@@@@@@@@&
              &@@@@@@@@@@@@@@
                     ..."##########
    )
}

pub fn gnu() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}    _-`````-,           ,- '- .
  .'   .- - |          | - -.  `.
 /.'  /                     `.   \
:/   :      _...   ..._      ``   :
::   :     /._ .`:'_.._\.    ||   :
::    `._ ./  ,`  :    \ . _.''   .
`:.      /   |  -.  \-. \\_      /
  \:._ _/  .'   .@)  \@) ` `\ ,.'
     _/,--'       .- .\,-.`--`.
       ,'/''     (( \ `  )
        /'/'  \    `-'  (
         '/''  `._,-----'
          ''/'    .,---'
           ''/'      ;:
             ''/''  ''/
               ''/''/''
                 '/'/'
                  `;"##########
    )
}

pub fn gobo_linux() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}  _____       _
 / ____|     | |
| |  __  ___ | |__   ___
| | |_ |/ _ \| '_ \ / _ \
| |__| | (_) | |_) | (_) |
 \_____|\___/|_.__/ \___/"##########
    )
}

pub fn grombyang() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[2];
    let c3 = COLORS[1];
    format!(
        r##########"{c1}            eeeeeeeeeeee
         eeeeeeeeeeeeeeeee
      eeeeeeeeeeeeeeeeeeeeeee
    eeeee       {c2}.o+       {c1}eeee
  eeee         {c2}`ooo/         {c1}eeee
 eeee         {c2}`+oooo:         {c1}eeee
eee          {c2}`+oooooo:          {c1}eee
eee          {c2}-+oooooo+:         {c1}eee
ee         {c2}`/:oooooooo+:         {c1}ee
ee        {c2}`/+   +++    +:        {c1}ee
ee              {c2}+o+\             {c1}ee
eee             {c2}+o+\            {c1}eee
eee        {c2}//  \\ooo/  \\\        {c1}eee
 eee      {c2}//++++oooo++++\\\     {c1}eee
  eeee    {c2}::::++oooo+:::::   {c1}eeee
    eeeee   {c3}Grombyang OS {c1}  eeee
      eeeeeeeeeeeeeeeeeeeeeee
         eeeeeeeeeeeeeeeee"##########
    )
}

pub fn guix_small() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}|.__          __.|
|__ \\        / __|
   \\ \\      / /
    \\ \\    / /
     \\ \\  / /
      \\ \\/ /
       \\__/"##########
    )
}

pub fn guix() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1} ..                             `.
 `--..```..`           `..```..--`
   .-:///-:::.       `-:::///:-.
      ````.:::`     `:::.````
           -//:`    -::-
            ://:   -::-
            `///- .:::`
             -+++-:::.
              :+/:::-
              `-....`"##########
    )
}

pub fn haiku_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}       ,^,
      /   \\
*--_ ;     ; _--*
\\   '"     "'   /
 '.           .'
.-'"         "'-.
 '-.__.   .__.-'
       |_|"##########
    )
}

pub fn haiku() -> String {
    let c3 = COLORS[7];
    let c4 = COLORS[2];
    format!(
        r##########"{c3}

           MMMM              MMMM
           MMMM              MMMM
           MMMM              MMMM
           MMMM              MMMM
           MMMM{c4}       .ciO| /YMMMMM*"
{c3}           MMMM{c4}   .cOMMMMM|/MMMMM/`
 ,         ,iMM|/MMMMMMMMMMMMMMM*
  `*.__,-cMMMMMMMMMMMMMMMMM/`{c3}.MMM
           MM{c4}MMMMMMM/`:MMM/  {c3}MMMM
           MMMM              MMMM
           MMMM              MMMM
           """"              """""##########
    )
}

pub fn huayra() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                     `
            .       .       `
       ``    -      .      .
        `.`   -` `. -  `` .`
          ..`-`-` + -  / .`     ```
          .--.+--`+:- :/.` .-``.`
            -+/so::h:.d-`./:`.`
              :hNhyMomy:os-...-.  ````
               .dhsshNmNhoo+:-``.```
                {c1}`ohy:-{c2}NMds+::-.``
            ````{c1}.hNN+`{c2}mMNho/:-....````
       `````     `../dmNhoo+/:..``
    ````            .dh++o/:....`
.+s/`                `/s-.-.:.`` ````
::`                    `::`..`
                          .` `..
                                ``"##########
    )
}

pub fn hydro_os() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}
  _    _           _            ____   _____
 | |  | |         | |          / __ \ / ____|
 | |__| |_   _  __| |_ __ ___ | |  | | (___
 |  __  | | | |/ _` | '__/ _ \| |  | |\___ \
 | |  | | |_| | (_| | | | (_) | |__| |____) |
 |_|  |_|\__, |\__,_|_|  \___/ \____/|_____/
          __/ |
         |___/"##########
    )
}

pub fn hyperbola_small() -> String {
    let c1 = COLORS[8];
    format!(
        r##########"{c1}    |`__.`/
    \____/
    .--.
   /    \\
  /  ___ \\
 / .`   `.\\
/.`      `.\\"##########
    )
}

pub fn hyperbola() -> String {
    let c1 = COLORS[8];
    format!(
        r##########"{c1}                     WW
                     KX              W
                    WO0W          NX0O
                    NOO0NW  WNXK0OOKW
                    W0OOOOOOOOOOOOKN
                     N0OOOOOOO0KXW
                       WNXXXNW
                 NXK00000KN
             WNK0OOOOOOOOOO0W
           NK0OOOOOOOOOOOOOO0W
         X0OOOOOOO00KK00OOOOOK
       X0OOOO0KNWW      WX0OO0W
     X0OO0XNW              KOOW
   N00KNW                   KOW
 NKXN                       W0W
WW                           W"##########
    )
}

pub fn iglunix() -> String {
    let c1 = COLORS[8];
    format!(
        r##########"{c1}     |
     |        |
              |
|    ________
|  /\   |    \
  /  \  |     \  |
 /    \        \ |
/      \________\
\      /        /
 \    /        /
  \  /        /
   \/________/"##########
    )
}

pub fn januslinux() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[5];
    format!(
        r##########"{c1}               'l:
        loooooo
          loooo coooool
 looooooooooooooooooool
  looooooooooooooooo
         lool   cooo
        coooooooloooooooo
     clooooo  ;lood  cloooo
  :loooocooo cloo      loooo
 loooo  :ooooool       loooo
looo    cooooo        cooooo
looooooooooooo      ;loooooo {c2}looooooc
{c1}looooooooo loo   cloooooool    {c2}looooc
{c1} cooo       cooooooooooo       {c2}looolooooool
{c1}            cooo:     {c2}coooooooooooooooooool
                       loooooooooooolc:   loooc;
                             cooo:    loooooooooooc
                            ;oool         looooooo:
                           coool          olc,
                          looooc   ,,
                        coooooc    loc
                       :oooool,    coool:, looool:,
                       looool:      ooooooooooooooo:
                       cooolc        .ooooooooooool"##########
    )
}

pub fn kaisen() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}                          `
                  `:+oyyho.
             `+:`sdddddd/
        `+` :ho oyo++ohds-`
       .ho :dd.  .: `sddddddhhyso+/-
       ody.ddd-:yd- +hysssyhddddddddho`
       yddddddhddd` ` `--`   -+hddddddh.
       hddy-+dddddy+ohh/..+sddddy/:::+ys
      :ddd/sdddddddddd- oddddddd       `
     `yddddddddddddddd/ /ddddddd/
:.  :ydddddddddddddddddo..sddddddy/`
odhdddddddo- `ddddh+-``....-+hdddddds.
-ddddddhd:   /dddo  -ydddddddhdddddddd-
 /hdy:o - `:sddds   .`./hdddddddddddddo
  `/-  `+hddyosy+       :dddddddy-.-od/
      :sydds           -hddddddd`    /
       .+shd-      `:ohddddddddd`
                `:+ooooooooooooo:"##########
    )
}

pub fn kali() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[8];
    format!(
        r##########"{c1}..............
            ..,;:ccc,.
          ......''';lxO.
.....''''..........,:ld;
           .';;;:::;,,.x,
      ..'''.            0Xxoc:,.  ...
  ....                ,ONkc;,;cokOdc',.
 .                   OMo           ':{c2}dd{c1}o.
                    dMc               :OO;
                    0M.                 .:o.
                    ;Wd
                     ;XO,
                       ,d0Odlc;,..
                           ..',;:cdOOd::,.
                                    .:d;.':;.
                                       'd,  .'
                                         ;l   ..
                                          .o
                                            c
                                            .'
                                             ."##########
    )
}

pub fn ka_os() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                     ..
  .....         ..OSSAAAAAAA..
 .KKKKSS.     .SSAAAAAAAAAAA.
.KKKKKSO.    .SAAAAAAAAAA...
KKKKKKS.   .OAAAAAAAA.
KKKKKKS.  .OAAAAAA.
KKKKKKS. .SSAA..
.KKKKKS..OAAAAAAAAAAAA........
 DKKKKO.=AA=========A===AASSSO..
  AKKKS.==========AASSSSAAAAAASS.
  .=KKO..========ASS.....SSSSASSSS.
    .KK.       .ASS..O.. =SSSSAOSS:
     .OK.      .ASSSSSSSO...=A.SSA.
       .K      ..SSSASSSS.. ..SSA.
                 .SSS.AAKAKSSKA.
                    .SSS....S.."##########
    )
}

pub fn kde() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}             `..---+/---..`
         `---.``   ``   `.---.`
      .--.`        ``        `-:-.
    `:/:     `.----//----.`     :/-
   .:.    `---`          `--.`    .:`
  .:`   `--`                .:-    `:.
 `/    `:.      `.-::-.`      -:`   `/`
 /.    /.     `:++++++++:`     .:    .:
`/    .:     `+++++++++++/      /`   `+`
/+`   --     .++++++++++++`     :.   .+:
`/    .:     `+++++++++++/      /`   `+`
 /`    /.     `:++++++++:`     .:    .:
 ./    `:.      `.:::-.`      -:`   `/`
  .:`   `--`                .:-    `:.
   .:.    `---`          `--.`    .:`
    `:/:     `.----//----.`     :/-
      .-:.`        ``        `-:-.
         `---.``   ``   `.---.`
             `..---+/---..`"##########
    )
}

pub fn kibojoe() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    let c3 = COLORS[4];
    format!(
        r##########"{c3}           ./+oooooo+/.
           -/+ooooo+/:.`
          {c1}`{c3}yyyo{c2}+++/++{c3}osss{c1}.
         {c1}+NMN{c3}yssssssssssss{c1}.
       {c1}.dMMMMN{c3}sssssssssssy{c1}Ns`
      +MMMMMMMm{c3}sssssssssssh{c1}MNo`
    `hMMMMMNNNMd{c3}sssssssssssd{c1}MMN/
   .{c3}syyyssssssy{c1}NNmmmmd{c3}sssss{c1}hMMMMd:
  -NMmh{c3}yssssssssyhhhhyssyh{c1}mMMMMMMMy`
 -NMMMMMNN{c3}mdhyyyyyyyhdm{c1}NMMMMMMMMMMMN+
`NMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMd.
ods+/:-----://+oyydmNMMMMMMMMMMMMMMMMMN-
`                     .-:+osyhhdmmNNNmdo"##########
    )
}

pub fn kogaion() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}            ;;      ,;
           ;;;     ,;;
         ,;;;;     ;;;;
      ,;;;;;;;;    ;;;;
     ;;;;;;;;;;;   ;;;;;
    ,;;;;;;;;;;;;  ';;;;;,
    ;;;;;;;;;;;;;;, ';;;;;;;
    ;;;;;;;;;;;;;;;;;, ';;;;;
;    ';;;;;;;;;;;;;;;;;;, ;;;
;;;,  ';;;;;;;;;;;;;;;;;;;,;;
;;;;;,  ';;;;;;;;;;;;;;;;;;,
;;;;;;;;,  ';;;;;;;;;;;;;;;;,
;;;;;;;;;;;;, ';;;;;;;;;;;;;;
';;;;;;;;;;;;; ';;;;;;;;;;;;;
 ';;;;;;;;;;;;;, ';;;;;;;;;;;
  ';;;;;;;;;;;;;  ;;;;;;;;;;
    ';;;;;;;;;;;; ;;;;;;;;
        ';;;;;;;; ;;;;;;
           ';;;;; ;;;;
             ';;; ;;"##########
    )
}

pub fn korora() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                ____________
             _add55555555554{c1}:
           _w?'{c1}``````````'{c2})k{c1}:
          _Z'{c1}`{c2}            ]k{c1}:
          m({c1}`{c2}             )k{c1}:
     _.ss{c1}`{c2}m[{c1}`{c2},            ]e{c1}:
   .uY"^`{c1}`{c2}Xc{c1}`{c2}?Ss.         d({c1}`
  jF'{c1}`{c2}    `@.  {c1}`{c2}Sc      .jr{c1}`
 jr{c1}`{c2}       `?n_ {c1}`{c2}$;   _a2"{c1}`
.m{c1}:{c2}          `~M{c1}`{c2}1k{c1}`{c2}5?!`{c1}`
:#{c1}:{c2}             {c1}`{c2})e{c1}```
:m{c1}:{c2}             ,#'{c1}`
:#{c1}:{c2}           .s2'{c1}`
:m,________.aa7^{c1}`
:#baaaaaaas!J'{c1}`
 ```````````"##########
    )
}

pub fn ks_linux() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1} K   K U   U RRRR   ooo
 K  K  U   U R   R o   o
 KKK   U   U RRRR  o   o
 K  K  U   U R  R  o   o
 K   K  UUU  R   R  ooo

{c2}  SSS   AAA  W   W  AAA
 S     A   A W   W A   A
  SSS  AAAAA W W W AAAAA
     S A   A WW WW A   A
  SSS  A   A W   W A   A"##########
    )
}

pub fn kubuntu() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}           `.:/ossyyyysso/:.
        .:oyyyyyyyyyyyyyyyyyyo:`
      -oyyyyyyyo{c2}dMMy{c1}yyyyyyysyyyyo-
    -syyyyyyyyyy{c2}dMMy{c1}oyyyy{c2}dmMMy{c1}yyyys-
   oyyys{c2}dMy{c1}syyyy{c2}dMMMMMMMMMMMMMy{c1}yyyyyyo
 `oyyyy{c2}dMMMMy{c1}syysoooooo{c2}dMMMMy{c1}yyyyyyyyo`
 oyyyyyy{c2}dMMMMy{c1}yyyyyyyyyyys{c2}dMMy{c1}sssssyyyo
-yyyyyyyy{c2}dMy{c1}syyyyyyyyyyyyyys{c2}dMMMMMy{c1}syyy-
oyyyysoo{c2}dMy{c1}yyyyyyyyyyyyyyyyyy{c2}dMMMMy{c1}syyyo
yyys{c2}dMMMMMy{c1}yyyyyyyyyyyyyyyyyysosyyyyyyyy
yyys{c2}dMMMMMy{c1}yyyyyyyyyyyyyyyyyyyyyyyyyyyyy
oyyyyysos{c2}dy{c1}yyyyyyyyyyyyyyyyyy{c2}dMMMMy{c1}syyyo
-yyyyyyyy{c2}dMy{c1}syyyyyyyyyyyyyys{c2}dMMMMMy{c1}syyy-
 oyyyyyy{c2}dMMMy{c1}syyyyyyyyyyys{c2}dMMy{c1}oyyyoyyyo
 `oyyyy{c2}dMMMy{c1}syyyoooooo{c2}dMMMMy{c1}oyyyyyyyyo
   oyyysyyoyyyys{c2}dMMMMMMMMMMMy{c1}yyyyyyyo
    -syyyyyyyyy{c2}dMMMy{c1}syyy{c2}dMMMy{c1}syyyys-
      -oyyyyyyy{c2}dMMy{c1}yyyyyysosyyyyo-
        ./oyyyyyyyyyyyyyyyyyyo/.
           `.:/oosyyyysso/:.`"##########
    )
}

pub fn lede() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1} _________
    /        /\
   /  LE    /  \
  /    DE  /    \
 /________/  LE  \
 \        \   DE /
  \    LE  \    /
   \  DE    \  /
    \________\/"##########
    )
}

pub fn laxer_os() -> String {
    let c2 = COLORS[4];
    format!(
        r##########"{c2}
                    /.
                 `://:-
                `//////:
               .////////:`
              -//////////:`
             -/////////////`
            :///////////////.
          `://////.```-//////-
         `://///:`     .//////-
        `//////:        `//////:
       .//////-          `://///:`
      -//////-            `://///:`
     -//////.               ://////`
    ://////`                 -//////.
   `/////:`                   ./////:
    .-::-`                     .:::-`

.:://////////////////////////////////::.
////////////////////////////////////////
.:////////////////////////////////////:."##########
    )
}

pub fn libre_elec() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[3];
    let c3 = COLORS[7];
    let c4 = COLORS[14];
    let c5 = COLORS[13];
    format!(
        r##########"{c1}          :+ooo/.      {c2}./ooo+:
{c1}        :+ooooooo/.  {c2}./ooooooo+:
{c1}      :+ooooooooooo:{c2}:ooooooooooo+:
{c1}    :+ooooooooooo+-  {c2}-+ooooooooooo+:
{c1}  :+ooooooooooo+-  {c3}--  {c2}-+ooooooooooo+:
{c1}.+ooooooooooo+-  {c3}:+oo+:  {c2}-+ooooooooooo+-
{c1}-+ooooooooo+-  {c3}:+oooooo+:  {c2}-+oooooooooo-
{c1}  :+ooooo+-  {c3}:+oooooooooo+:  {c2}-+oooooo:
{c1}    :+o+-  {c3}:+oooooooooooooo+:  {c2}-+oo:
{c4}     ./   {c3}:oooooooooooooooooo:   {c5}/.
{c4}   ./oo+:  {c3}-+oooooooooooooo+-  {c5}:+oo/.
{c4} ./oooooo+:  {c3}-+oooooooooo+-  {c5}:+oooooo/.
{c4}-oooooooooo+:  {c3}-+oooooo+-  {c5}:+oooooooooo-
{c4}.+ooooooooooo+:  {c3}-+oo+-  {c5}:+ooooooooooo+.
{c4}  -+ooooooooooo+:  {c3}..  {c5}:+ooooooooooo+-
{c4}    -+ooooooooooo+:  {c5}:+ooooooooooo+-
{c4}      -+oooooooooo+:{c5}:+oooooooooo+-
{c4}        -+oooooo+:    {c5}:+oooooo+-
{c4}          -+oo+:        {c5}:+oo+-
{c4}            ..            {c5}.."##########
    )
}

pub fn linux() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[8];
    let c3 = COLORS[3];
    format!(
        r##########"{c2}        #####
{c2}       #######
{c2}       ##{c1}O{c2}#{c1}O{c2}##
{c2}       #{c3}#####{c2}#
{c2}     ##{c1}##{c3}###{c1}##{c2}##
{c2}    #{c1}##########{c2}##
{c2}   #{c1}############{c2}##
{c2}   #{c1}############{c2}###
{c3}  ##{c2}#{c1}###########{c2}##{c3}#
{c3}######{c2}#{c1}#######{c2}#{c3}######
{c3}#######{c2}#{c1}#####{c2}#{c3}#######
{c3}  #####{c2}#######{c3}#####"##########
    )
}

pub fn linuxlite_small() -> String {
    let c1 = COLORS[3];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}   /\\
  /  \\
 / {c2}/ {c1}/
> {c2}/ {c1}/
\\ {c2}\\ {c1}\\
 \\_{c2}\\{c1}_\\
{c2}    \\"##########
    )
}

pub fn linux_lite() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}          ,xXc
      .l0MMMMMO
   .kNMMMMMWMMMN,
   KMMMMMMKMMMMMMo
  'MMMMMMNKMMMMMM:
  kMMMMMMOMMMMMMO
 .MMMMMMX0MMMMMW.
 oMMMMMMxWMMMMM:
 WMMMMMNkMMMMMO
:MMMMMMOXMMMMW
.0MMMMMxMMMMM;
:;cKMMWxMMMMO
'MMWMMXOMMMMl
 kMMMMKOMMMMMX:
 .WMMMMKOWMMM0c
  lMMMMMWO0MNd:'
   oollXMKXoxl;.
     ':. .: .'
              ..
                ."##########
    )
}

pub fn lmde() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}`.-::---..
{c1}      .:++++ooooosssoo:.
    .+o++::.      `.:oos+.
{c1}   :oo:.`             -+oo{c2}:
{c1} {c2}`{c1}+o/`    .{c2}::::::{c1}-.    .++-{c2}`
{c1}{c2}`{c1}/s/    .yyyyyyyyyyo:   +o-{c2}`
{c1}{c2}`{c1}so     .ss       ohyo` :s-{c2}:
{c1}{c2}`{c1}s/     .ss  h  m  myy/ /s`{c2}`
{c1}`s:     `oo  s  m  Myy+-o:`
`oo      :+sdoohyoydyso/.
 :o.      .:////////++:
{c1} `/++        {c2}-:::::-
{c1}  {c2}`{c1}++-
{c1}   {c2}`{c1}/+-
{c1}     {c2}.{c1}+/.
{c1}       {c2}.{c1}:+-.
          `--.``"##########
    )
}

pub fn lubuntu() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}           `.:/ossyyyysso/:.
        `.:yyyyyyyyyyyyyyyyyy:.`
      .:yyyyyyyyyyyyyyyyyyyyyyyy:.
    .:yyyyyyyyyyyyyyyyyyyyyyyyyyyy:.
   -yyyyyyyyyyyyyy{c2}+hNMMMNh+{c1}yyyyyyyyy-
  :yy{c2}mNy+{c1}yyyyyyyy{c2}+Nmso++smMdhyysoo+{c1}yy:
 -yy{c2}+MMMmmy{c1}yyyyyy{c2}hh{c1}yyyyyyyyyyyyyyyyyyy-
.yyyy{c2}NMN{c1}yy{c2}shhs{c1}yyy{c2}+o{c1}yyyyyyyyyyyyyyyyyyyy.
:yyyy{c2}oNM+{c1}yyyy{c2}+sso{c1}yyyyyyy{c2}ss{c1}yyyyyyyyyyyyy:
:yyyyy{c2}+dNs{c1}yyyyyyy{c2}++{c1}yyyyy{c2}oN+{c1}yyyyyyyyyyyy:
:yyyyy{c2}oMMmhysso{c1}yyyyyyyyyy{c2}mN+{c1}yyyyyyyyyyy:
:yyyyyy{c2}hMm{c1}yyyyy{c2}+++{c1}yyyyyyy{c2}+MN{c1}yyyyyyyyyyy:
.yyyyyyy{c2}ohmy+{c1}yyyyyyyyyyyyy{c2}NMh{c1}yyyyyyyyyy.
 -yyyyyyyyyy{c2}++{c1}yyyyyyyyyyyy{c2}MMh{c1}yyyyyyyyy-
  :yyyyyyyyyyyyyyyyyyyyy{c2}+mMN+{c1}yyyyyyyy:
   -yyyyyyyyyyyyyyyyy{c2}+sdMMd+{c1}yyyyyyyy-
    .:yyyyyyyyy{c2}hmdmmNMNdy+{c1}yyyyyyyy:.
      .:yyyyyyy{c2}my{c1}yyyyyyyyyyyyyyy:.
        `.:yyyy{c2}s{c1}yyyyyyyyyyyyy:.`
           `.:/oosyyyysso/:.`"##########
    )
}

pub fn lunar() -> String {
    let c1 = COLORS[4];
    let c3 = COLORS[3];
    format!(
        r##########"{c1}`-.                                 `-.
  -ohys/-`                    `:+shy/`
     -omNNdyo/`          :+shmNNy/`
             {c3}      -
                 /mMmo
                 hMMMN`
                 .NMMs
    {c1}  -:+oooo+//: {c3}/MN{c1}. -///oooo+/-`
     /:.`          {c3}/{c1}           `.:/`
{c3}          __
         |  |   _ _ ___ ___ ___
         |  |__| | |   | .'|  _|
         |_____|___|_|_|__,|_|"##########
    )
}

pub fn mac() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[3];
    let c3 = COLORS[1];
    let c4 = COLORS[5];
    let c5 = COLORS[4];
    format!(
        r##########"{c1}       .:'
    _ :'_
{c2} .'`_`-'_``.
:________.-'
{c3}:_______:
:_______:
{c4} :_______`-;
{c5}  `._.-._.'"##########
    )
}

pub fn mageia_small() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}   *
    *
   **
{c2} /\\__/\\
/      \\
\\      /
 \\____/"##########
    )
}

pub fn mageia() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}        .°°.
         °°   .°°.
         .°°°. °°
         .   .
          °°° .°°°.
      .°°°.   '___'
{c2}     .{c1}'___'     {c2}   .
   :dkxc;'.  ..,cxkd;
 .dkk. kkkkkkkkkk .kkd.
.dkk.  ';cloolc;.  .kkd
ckk.                .kk;
xO:                  cOd
xO:                  lOd
lOO.                .OO:
.k00.              .00x
 .k00;            ;00O.
  .lO0Kc;,,,,,,;c0KOc.
     ;d00KKKKKK00d;
        .,KKKK,."##########
    )
}

pub fn magpie_os() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[1];
    let c3 = COLORS[3];
    let c4 = COLORS[5];
    format!(
        r##########"{c1}        ;00000     :000Ol
     .x00kk00:    O0kk00k;
    l00:   :00.  o0k   :O0k.
  .k0k.     x{c2}d$dddd{c1}k'    .d00;
  k0k.      {c2}.dddddl       {c1}o00,
 o00.        {c2}':cc:.        {c1}d0O
.00l                       ,00.
l00.                       d0x
k0O                     .:k0o
O0k                 ;dO0000d.
k0O               .O0O{c2}xxxxk{c1}00:
o00.              k0O{c2}dddddd{c1}occ
'00l              x0O{c2}dddddo{c3};..{c1}
 x00.             .x00{c2}kxxd{c3}:..{c1}
 .O0x               .:oxxx{c4}Okl.{c1}
  .x0d                     {c4},xx,{c1}
    .:o.          {c4}.xd       ckd{c1}
       ..          {c4}dxl     .xx;
                    :xxolldxd'
                      ;oxdl."##########
    )
}

pub fn mandriva() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[3];
    format!(
        r##########"{c2}                        ``
                       `-.
{c1}      `               {c2}.---
{c1}    -/               {c2}-::--`
{c1}  `++    {c2}`----...```-:::::.
{c1} `os.      {c2}.::::::::::::::-```     `  `
{c1} +s+         {c2}.::::::::::::::::---...--`
{c1}-ss:          {c2}`-::::::::::::::::-.``.``
{c1}/ss-           {c2}.::::::::::::-.``   `
{c1}+ss:          {c2}.::::::::::::-
{c1}/sso         {c2}.::::::-::::::-
{c1}.sss/       {c2}-:::-.`   .:::::
{c1} /sss+.    {c2}..`{c1}  `--`    {c2}.:::
{c1}  -ossso+/:://+/-`        {c2}.:`
{c1}    -/+ooo+/-.              {c2}`"##########
    )
}

pub fn manjaro_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}||||||||| ||||
||||||||| ||||
||||      ||||
|||| |||| ||||
|||| |||| ||||
|||| |||| ||||
|||| |||| ||||"##########
    )
}

pub fn manjaro() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
████████            ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████"##########
    )
}

// pub fn TeArch() -> String {
//     let c1 = COLORS[39];
//     let c2 = COLORS[7];
//     let c3 = COLORS[1];
//     format!(
//         r##########"{c1}          @@@@@@@@@@@@@@
//       @@@@@@@@@              @@@@@@
//      @@@@@                     @@@@@
//      @@                           @@
//       @%                         @@
//        @                         @
//        @@@@@@@@@@@@@@@@@@@@@@@@ @@
//        .@@@@@@@@@@@@/@@@@@@@@@@@@
//        @@@@@@@@@@@@///@@@@@@@@@@@@
//       @@@@@@@@@@@@@((((@@@@@@@@@@@@
//      @@@@@@@@@@@#(((((((#@@@@@@@@@@@
//     @@@@@@@@@@@#//////////@@@@@@@@@@&
//     @@@@@@@@@@////@@@@@////@@@@@@@@@@
//     @@@@@@@@//////@@@@@/////@@@@@@@@@
//     @@@@@@@//@@@@@@@@@@@@@@@//@@@@@@@
//  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@     .@@@@@@@@@@@@@@@@@@@@@@@@@      @
//  @@@@@@           @@@.           @@@@@@@
//    @@@@@@@&@@@@@@@#  #@@@@@@@@@@@@@@@@
//       @@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//           @@@@@@@@@@@@@@@@@@@@@"##########
//     )
// }

pub fn maui() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}             `.-://////:--`
         .:/oooooooooooooooo+:.
      `:+ooooooooooooooooooooooo:`
    `:oooooooooooooooooooooooooooo/`
    ..```-oooooo/-`` `:oooooo+:.` `--
  :.      +oo+-`       /ooo/`       -/
 -o.     `o+-          +o/`         -o:
`oo`     ::`  :o/     `+.  .+o`     /oo.
/o+      .  -+oo-     `   /oo/     `ooo/
+o-        /ooo+`       .+ooo.     :ooo+
++       .+oooo:       -oooo+     `oooo+
:.      .oooooo`      :ooooo-     :oooo:
`      .oooooo:      :ooooo+     `ooo+-`
      .+oooooo`     -oooooo:     `o/-
      +oooooo:     .ooooooo.
     /ooooooo`     /ooooooo/       ..
    `:oooooooo/:::/ooooooooo+:--:/:`
      `:+oooooooooooooooooooooo+:`
         .:+oooooooooooooooo+:.
             `.-://////:-.`"##########
    )
}

pub fn mer() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                         dMs
                         .-`
                       `y`-o+`
                        ``NMMy
                      .--`:++.
                    .hNNNNs
                    /MMMMMN
                    `ommmd/ +/
                      ````  +/
                     `:+sssso/-`
  .-::. `-::-`     `smNMNmdmNMNd/      .://-`
.ymNMNNdmNMMNm+`  -dMMh:.....+dMMs   `sNNMMNo
dMN+::NMMy::hMM+  mMMo `ohhy/ `dMM+  yMMy::-
MMm   yMM-  :MMs  NMN` `:::::--sMMh  dMM`
MMm   yMM-  -MMs  mMM+ `ymmdsymMMMs  dMM`
NNd   sNN-  -NNs  -mMNs-.--..:dMMh`  dNN
---   .--`  `--.   .smMMmdddmMNdo`   .--
                     ./ohddds+:`
                     +h- `.:-.
                     ./`.dMMMN+
                        +MMMMMd
                        `+dmmy-
                      ``` .+`
                     .dMNo-y.
                     `hmm/
                         .:`
                         dMs"##########
    )
}

pub fn minix() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}   -sdhyo+:-`                -/syymm:
   sdyooymmNNy.     ``    .smNmmdysNd
   odyoso+syNNmysoyhhdhsoomNmm+/osdm/
    :hhy+-/syNNmddhddddddmNMNo:sdNd:
     `smNNdNmmNmddddddddddmmmmmmmy`
   `ohhhhdddddmmNNdmddNmNNmdddddmdh-
   odNNNmdyo/:/-/hNddNy-`..-+ydNNNmd:
 `+mNho:`   smmd/ sNNh :dmms`   -+ymmo.
-od/       -m{c1}mm{c2}mo -NN+ +m{c1}mm{c2}m-       yms:
+sms -.`    :so:  .NN+  :os/     .-`mNh:
.-hyh+:////-     -sNNd:`    .--://ohNs-
 `:hNNNNNNNMMd/sNMmhsdMMh/ymmNNNmmNNy/
  -+sNNNNMMNNNsmNMo: :NNmymNNNNMMMms:
    //oydNMMMMydMMNysNMMmsMMMMMNyo/`
       ../-yNMMy--/::/-.sMMmos+.`
           -+oyhNsooo+omy/```
              `::ohdmds-`"##########
    )
}

pub fn linuxmint_small() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c1} ___________
|_          \\
  | {c2}| _____ {c1}|
  | {c2}| | | | {c1}|
  | {c2}| | | | {c1}|
  | {c2}\\__{c2}___/ {c1}|
  \\_________/"##########
    )
}

pub fn linux_mint_old() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}MMMMMMMMMMMMMMMMMMMMMMMMMmds+.
MMm----::-://////////////oymNMd+`
MMd      {c2}/++                {c1}-sNMd:
MMNso/`  {c2}dMM    `.::-. .-::.` {c1}.hMN:
ddddMMh  {c2}dMM   :hNMNMNhNMNMNh: {c1}`NMm
    NMm  {c2}dMM  .NMN/-+MMM+-/NMN` {c1}dMM
    NMm  {c2}dMM  -MMm  `MMM   dMM. {c1}dMM
    NMm  {c2}dMM  -MMm  `MMM   dMM. {c1}dMM
    NMm  {c2}dMM  .mmd  `mmm   yMM. {c1}dMM
    NMm  {c2}dMM`  ..`   ...   ydm. {c1}dMM
    hMM- {c2}+MMd/-------...-:sdds  {c1}dMM
    -NMm- {c2}:hNMNNNmdddddddddy/`  {c1}dMM
     -dMNs-{c2}``-::::-------.``    {c1}dMM
      `/dMNmy+/:-------------:/yMMM
         ./ydNMMMMMMMMMMMMMMMMMMMMM
            .MMMMMMMMMMMMMMMMMMM"##########
    )
}

pub fn linux_mint() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}             ...-:::::-...
{c2}          .-MMMMMMMMMMMMMMM-.
      .-MMMM{c1}`..-:::::::-..`{c2}MMMM-.
    .:MMMM{c1}.:MMMMMMMMMMMMMMM:.{c2}MMMM:.
   -MMM{c1}-M---MMMMMMMMMMMMMMMMMMM.{c2}MMM-
 `:MMM{c1}:MM`  :MMMM:....::-...-MMMM:{c2}MMM:`
 :MMM{c1}:MMM`  :MM:`  ``    ``  `:MMM:{c2}MMM:
.MMM{c1}.MMMM`  :MM.  -MM.  .MM-  `MMMM.{c2}MMM.
:MMM{c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{c2}MMM:
:MMM{c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM:{c2}MMM:
:MMM{c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{c2}MMM:
.MMM{c1}.MMMM`  :MM:--:MM:--:MM:  `MMMM.{c2}MMM.
 :MMM{c1}:MMM-  `-MMMMMMMMMMMM-`  -MMM-{c2}MMM:
  :MMM{c1}:MMM:`                `:MMM:{c2}MMM:
   .MMM{c1}.MMMM:--------------:MMMM.{c2}MMM.
     '-MMMM{c1}.-MMMMMMMMMMMMMMM-.{c2}MMMM-'
       '.-MMMM{c1}``--:::::--``{c2}MMMM-.'
{c2}            '-MMMMMMMMMMMMM-'
{c2}               ``-:::::-``"##########
    )
}

pub fn live_raizo() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}             `......`
        -+shmNMMMMMMNmhs/.
     :smMMMMMmmhyyhmmMMMMMmo-
   -hMMMMd+:. `----` .:odMMMMh-
 `hMMMN+. .odNMMMMMMNdo. .yMMMMs`
 hMMMd. -dMMMMmdhhdNMMMNh` .mMMMh
oMMMm` :MMMNs.:sddy:-sMMMN- `NMMM+
mMMMs  dMMMo sMMMMMMd yMMMd  sMMMm
----`  .---` oNMMMMMh `---.  .----
              .sMMy:
               /MM/
              +dMMms.
             hMMMMMMN
            `dMMMMMMm:
      .+ss+sMNysMMoomMd+ss+.
     +MMMMMMN` +MM/  hMMMMMNs
     sMMMMMMm-hNMMMd-hMMMMMMd
      :yddh+`hMMMMMMN :yddy/`
             .hMMMMd:
               `..`"##########
    )
}

pub fn mx_small() -> String {
    let c3 = COLORS[7];
    format!(
        r##########"{c3}    \\\\  /
     \\\\/
      \\\\
   /\\/ \\\\
  /  \\  /\\
 /    \\/  \\
/__________\\"##########
    )
}

pub fn mx() -> String {
    let c3 = COLORS[7];
    format!(
        r##########"{c3}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNMMMMMMMMM
MMMMMMMMMMNs..yMMMMMMMMMMMMMm: +NMMMMMMM
MMMMMMMMMN+    :mMMMMMMMMMNo` -dMMMMMMMM
MMMMMMMMMMMs.   `oNMMMMMMh- `sNMMMMMMMMM
MMMMMMMMMMMMN/    -hMMMN+  :dMMMMMMMMMMM
MMMMMMMMMMMMMMh-    +ms. .sMMMMMMMMMMMMM
MMMMMMMMMMMMMMMN+`   `  +NMMMMMMMMMMMMMM
MMMMMMMMMMMMMMNMMd:    .dMMMMMMMMMMMMMMM
MMMMMMMMMMMMm/-hMd-     `sNMMMMMMMMMMMMM
MMMMMMMMMMNo`   -` :h/    -dMMMMMMMMMMMM
MMMMMMMMMd:       /NMMh-   `+NMMMMMMMMMM
MMMMMMMNo`         :mMMN+`   `-hMMMMMMMM
MMMMMMh.            `oNMMd:    `/mMMMMMM
MMMMm/                -hMd-      `sNMMMM
MMNs`                   -          :dMMM
Mm:                                 `oMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"##########
    )
}

pub fn namib() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}          .:+shysyhhhhysyhs+:.
       -/yyys              syyy/-
     -shy                      yhs-
   -yhs                          shy-
  +hy                              yh+
 +ds                                sd+
/ys                  so              sy/
sh                 smMMNdyo           hs
yo               ymMMMMNNMMNho        oy
N             ydMMMNNMMMMMMMMMmy       N
N         shmMMMMNNMMMMMMMMMMMMMNy     N
yo  ooshmNMMMNNNNMMMMMMMMMMMMMMMMMms  oy
sd yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy ds
/ys                                  sy/
 +ds                                sd+
  +hy                              yh+
   -yhs                          shy-
     -shy                      yhs-
       -/yyys              syyy/-
          .:+shysyhyhhysyhs+:."##########
    )
}

pub fn neptune() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}            ./+sydddddddys/-.
        .+ymNNdyooo/:+oooymNNmy/`
     `/hNNh/.`             `-+dNNy:`
    /mMd/.          .++.:oy/   .+mMd-
  `sMN/             oMMmdy+.     `oNNo
 `hMd.           `/ymy/.           :NMo
 oMN-          `/dMd:               /MM-
`mMy          -dMN+`                 mMs
.MMo         -NMM/                   yMs
 dMh         mMMMo:`                `NMo
 /MM/        /ymMMMm-               sMN.
  +Mm:         .hMMd`              oMN/
   +mNs.      `yNd/`             -dMm-
    .yMNs:    `/.`            `/yNNo`
      .odNNy+-`           .:ohNNd/.
         -+ymNNmdyyyyyyydmNNmy+.
             `-//sssssss//."##########
    )
}

pub fn netbsd_small() -> String {
    let c1 = COLORS[5];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}\\\\{c1}\`-______,----__
{c2} \\\\        {c1}__,---\`_
{c2}  \\\\       {c1}\`.____
{c2}   \\\\{c1}-______,----\`-
{c2}    \\\\
     \\\\
      \\\\"##########
    )
}

pub fn net_bsd() -> String {
    let c1 = COLORS[5];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}                     `-/oshdmNMNdhyo+:-`
{c2}y{c1}/s+:-``    `.-:+oydNMMMMNhs/-``
{c2}-m+{c1}NMMMMMMMMMMMMMMMMMMMNdhmNMMMmdhs+/-`
 {c2}-m+{c1}NMMMMMMMMMMMMMMMMMMMMmy+:`
  {c2}-N/{c1}dMMMMMMMMMMMMMMMds:`
   {c2}-N/{c1}hMMMMMMMMMmho:`
    {c2}-N/{c1}-:/++/:.`
{c2}     :M+
      :Mo
       :Ms
        :Ms
         :Ms
          :Ms
           :Ms
            :Ms
             :Ms
              :Ms"##########
    )
}

pub fn netrunner() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}           .:oydmMMMMMMmdyo:`
        -smMMMMMMMMMMMMMMMMMMds-
      +mMMMMMMMMMMMMMMMMMMMMMMMMd+
    /mMMMMMMMMMMMMMMMMMMMMMMMMMMMMm/
  `hMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMy`
 .mMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMd`
 dMMMMMMMMMMMMMMMMMMMMMMNdhmMMMMMMMMMMh
+MMMMMMMMMMMMMNmhyo+/-.   -MMMMMMMMMMMM/
mMMMMMMMMd+:.`           `mMMMMMMMMMMMMd
MMMMMMMMMMMdy/.          yMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMNh+`     +MMMMMMMMMMMMMMM
mMMMMMMMMMMMMMMMMMs    -NMMMMMMMMMMMMMMd
+MMMMMMMMMMMMMMMMMN.  `mMMMMMMMMMMMMMMM/
 dMMMMMMMMMMMMMMMMMy  hMMMMMMMMMMMMMMMh
 `dMMMMMMMMMMMMMMMMM-+MMMMMMMMMMMMMMMd`
  `hMMMMMMMMMMMMMMMMmMMMMMMMMMMMMMMMy
    /mMMMMMMMMMMMMMMMMMMMMMMMMMMMMm:
      +dMMMMMMMMMMMMMMMMMMMMMMMMd/
        -odMMMMMMMMMMMMMMMMMMdo-
           `:+ydmNMMMMNmhy+-`"##########
    )
}

pub fn nitrux() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}`:/.
`/yo
`/yo
`/yo      .+:.
`/yo      .sys+:.`
`/yo       `-/sys+:.`
`/yo           ./sss+:.`
`/yo              .:oss+:-`
`/yo                 ./o///:-`
`/yo              `.-:///////:`
`/yo           `.://///++//-``
`/yo       `.-:////++++/-`
`/yo    `-://///++o+/-`
`/yo `-/+o+++ooo+/-`
`/s+:+oooossso/.`
`//+sssssso:.
`+syyyy+:`
:+s+-"##########
    )
}

pub fn nixos_small() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}  \\\\  \\\\ //
 ==\\\\__\\\\/ //
   //   \\\\//
==//     //==
 //\\\\___//
// /\\\\  \\\\==
  // \\\\  \\\\"##########
    )
}

pub fn nixos_old() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[6];
    format!(
        r##########"{c1}          ::::.    {c2}':::::     ::::'
{c1}          ':::::    {c2}':::::.  ::::'
{c1}            :::::     {c2}'::::.:::::
{c1}      .......:::::..... {c2}::::::::
{c1}     ::::::::::::::::::. {c2}::::::    {c1}::::.
    ::::::::::::::::::::: {c2}:::::.  {c1}.::::'
{c2}           .....           ::::' {c1}:::::'
{c2}          :::::            '::' {c1}:::::'
{c2} ........:::::               ' {c1}:::::::::::.
{c2}:::::::::::::                 {c1}:::::::::::::
{c2} ::::::::::: {c1}..              {c1}:::::
{c2}     .::::: {c1}.:::            {c1}:::::
{c2}    .:::::  {c1}:::::          {c1}'''''    {c2}.....
    :::::   {c1}':::::.  {c2}......:::::::::::::'
     :::     {c1}::::::. {c2}':::::::::::::::::'
{c1}            .:::::::: {c2}'::::::::::
{c1}           .::::''::::.     {c2}'::::.
{c1}          .::::'   ::::.     {c2}'::::.
{c1}         .::::      ::::      {c2}'::::."##########
    )
}

pub fn nix_os() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[6];
    format!(
        r##########"{c1}          ▗▄▄▄       {c2}▗▄▄▄▄    ▄▄▄▖
{c1}          ▜███▙       {c2}▜███▙  ▟███▛
{c1}           ▜███▙       {c2}▜███▙▟███▛
{c1}            ▜███▙       {c2}▜██████▛
{c1}     ▟█████████████████▙ {c2}▜████▛     {c1}▟▙
{c1}    ▟███████████████████▙ {c2}▜███▙    {c1}▟██▙
{c2}           ▄▄▄▄▖           ▜███▙  {c1}▟███▛
{c2}          ▟███▛             ▜██▛ {c1}▟███▛
{c2}         ▟███▛               ▜▛ {c1}▟███▛
{c2}▟███████████▛                  {c1}▟██████████▙
{c2}▜██████████▛                  {c1}▟███████████▛
{c2}      ▟███▛ {c1}▟▙               ▟███▛
{c2}     ▟███▛ {c1}▟██▙             ▟███▛
{c2}    ▟███▛  {c1}▜███▙           ▝▀▀▀▀
{c2}    ▜██▛    {c1}▜███▙ {c2}▜██████████████████▛
{c2}     ▜▛     {c1}▟████▙ {c2}▜████████████████▛
{c1}           ▟██████▙       {c2}▜███▙
{c1}          ▟███▛▜███▙       {c2}▜███▙
{c1}         ▟███▛  ▜███▙       {c2}▜███▙
{c1}         ▝▀▀▀    ▀▀▀▀▘       {c2}▀▀▀▘"##########
    )
}

pub fn nurunner() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                  ,xc
                ;00cxXl
              ;K0,   .xNo.
            :KO'       .lXx.
          cXk.    ;xl     cXk.
        cXk.    ;k:.,xo.    cXk.
     .lXx.    :x::0MNl,dd.    :KO,
   .xNx.    cx;:KMMMMMNo'dx.    ;KK;
 .dNl.    cd,cXMMMMMMMMMWd,ox'    'OK:
;WK.    'K,.KMMMMMMMMMMMMMWc.Kx     lMO
 'OK:    'dl'xWMMMMMMMMMM0::x:    'OK:
   .kNo    .xo'xWMMMMMM0;:O:    ;KK;
     .dXd.   .do,oNMMO;ck:    ;00,
        oNd.   .dx,;'cO;    ;K0,
          oNx.    okk;    ;K0,
            lXx.        :KO'
              cKk'    cXk.
                ;00:lXx.
                  ,kd."##########
    )
}

pub fn nu_ty_x() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                                      .
                                    .
                                 ...
                               ...
            ....     .........--.
       ..-++-----....--++++++---.
    .-++++++-.   .-++++++++++++-----..
  .--...  .++..-+++--.....-++++++++++--..
 .     .-+-. .**-            ....  ..-+----..
     .+++.  .*+.         +            -++-----.
   .+++++-  ++.         .*+.     .....-+++-----.
  -+++-++. .+.          .-+***++***++--++++.  .
 -+-. --   -.          -*- ......        ..--.
.-. .+-    .          -+.
.  .+-                +.
   --                 --
  -+----.              .-
  -++-.+.                .
 .++. --
  +.  ----.
  .  .+. ..
      -  .
      ."##########
    )
}

pub fn ob_revenge() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}   __   __
     _@@@@   @@@g_
   _@@@@@@   @@@@@@
  _@@@@@@M   W@@@@@@_
 j@@@@P        ^W@@@@
 @@@@L____  _____Q@@@@
Q@@@@@@@@@@j@@@@@@@@@@
@@@@@    T@j@    T@@@@@
@@@@@ ___Q@J@    _@@@@@
@@@@@fMMM@@j@jggg@@@@@@
@@@@@    j@j@^MW@P @@@@
Q@@@@@ggg@@f@   @@@@@@L
^@@@@WWMMP  ^    Q@@@@
 @@@@@_         _@@@@l
  W@@@@@g_____g@@@@@P
   @@@@@@@@@@@@@@@@l
    ^W@@@@@@@@@@@P
       ^TMMMMTll"##########
    )
}

pub fn openbsd_small() -> String {
    let c1 = COLORS[3];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}      _____
    \\-     -/
 \\_/         \\
 |        {c2}O O{c1} |
 |_  <   )  3 )
 / \\         /
    /-_____-\\"##########
    )
}

pub fn open_bsd() -> String {
    let c1 = COLORS[3];
    let c3 = COLORS[6];
    let c4 = COLORS[1];
    let c5 = COLORS[8];
    format!(
        r##########"{c3}                                     _
                                    (_)
{c1}              |    .
{c1}          .   |L  /|   .         {c3} _
{c1}      _ . |\ _| \--+._/| .       {c3}(_)
{c1}     / ||\| Y J  )   / |/| ./
    J  |)'( |        ` F`.'/       {c3} _
{c1}  -<|  F         __     .-<        {c3}(_)
{c1}    | /       .-'{c3}. {c1}`.  /{c3}-. {c1}L___
    J \\      <    {c3}\ {c1} | | {c5}O{c3}\\{c1}|.-' {c3} _
{c1}  _J \\  .-    \\{c3}/ {c5}O {c3}| {c1}| \\  |{c1}F    {c3}(_)
{c1} '-F  -<_.     \\   .-'  `-' L__
__J  _   _.     >-'  {c1}){c4}._.   {c1}|-'
{c1} `-|.'   /_.          {c4}\_|  {c1} F
  /.-   .                _.<
 /'    /.'             .'  `\\
  /L  /'   |/      _.-'-\\
 /'J       ___.---'\|
   |\  .--' V  | `. `
   |/`. `-.     `._)
      / .-.\\
      \\ (  `\\
       `.\\"##########
    )
}

pub fn open_euler() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}                 `.cc.`
             ``.cccccccc..`
          `.cccccccccccccccc.`
      ``.cccccccccccccccccccccc.``
   `..cccccccccccccccccccccccccccc..`
`.ccccccccccccccc{c2}/++/{c1}ccccccccccccccccc.`
.ccccccccccccccc{c2}mNMMNdo+oso+{c1}ccccccccccc.
.cccccccccc{c2}/++odms+//+mMMMMm/:+syso/{c1}cccc
.ccccccccc{c2}yNNMMMs:::/::+o+/:{c1}c{c2}dMMMMMm{c1}cccc
.ccccccc{c2}:+NmdyyhNNmNNNd:{c1}ccccc{c1}{c2}:oyyyo:{c1}cccc
.ccc{c2}:ohdmMs:{c1}cccc{c2}+mNMNmy{c1}ccccccccccccccccc
.cc{c2}/NMMMMMo////:{c1}c{c2}:///:{c1}cccccccccccccccccc
.cc{c2}:syysyNMNNNMNy{c1}ccccccccccccccccccccccc
.cccccccc{c2}+MMMMMNy{c1}c{c2}:/+++/{c1}cccccccccccccccc
.ccccccccc{c2}ohhhs/{c1}c{c2}omMMMMNh{c1}ccccccccccccccc
.ccccccccccccccc{c2}:MMMMMMMM/{c1}cccccccccccccc
.cccccccccccccccc{c2}sNNNNNd+{c1}cccccccccccccc.
`..cccccccccccccccc{c2}/+/:{c1}cccccccccccccc..`
   ``.cccccccccccccccccccccccccccc.``
       `.cccccccccccccccccccccc.`
          ``.cccccccccccccc.``
              `.cccccccc.`
                 `....`"##########
    )
}

pub fn open_indiana() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                         .sy/
                         .yh+

           {c1}-+syyyo+-     {c2} /+.
         {c1}+ddo/---/sdh/   {c2} ym-
       {c1}`hm+        `sms{c2}   ym-```````.-.
       {c1}sm+           sm/ {c2} ym-         +s
       {c1}hm.           /mo {c2} ym-         /h
       {c1}omo           ym: {c2} ym-       `os`
        {c1}smo`       .ym+ {c2}  ym-     .os-
     ``  {c1}:ymy+///oyms- {c2}   ym-  .+s+.
   ..`     {c1}`:+oo+/-`  {c2}    -//oyo-
 -:`                   .:oys/.
+-               `./oyys/.
h+`      `.-:+oyyyo/-`
`/ossssysso+/-.`"##########
    )
}

pub fn openmamba() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[2];
    format!(
        r##########"{c1}                 `````
           .-/+ooooooooo+/:-`
        ./ooooooooooooooooooo+:.
      -+oooooooooooooooooooooooo+-
    .+ooooooooo+/:---::/+ooooooooo+.
   :oooooooo/-`          `-/oo{c2}s´{c1}oooo.{c2}s´{c1}
  :ooooooo/`                `{c2}sNds{c1}ooo{c2}sNds{c1}
 -ooooooo-                   {c2}:dmy{c1}ooo{c2}:dmy{c1}
 +oooooo:                      :oooooo-
.ooooooo                        .://:`
:oooooo+                        ./+o+:`
-ooooooo`                      `oooooo+
`ooooooo:                      /oooooo+
 -ooooooo:                    :ooooooo.
  :ooooooo+.                .+ooooooo:
   :oooooooo+-`          `-+oooooooo:
    .+ooooooooo+/::::://oooooooooo+.
      -+oooooooooooooooooooooooo+-
        .:ooooooooooooooooooo+:.
           `-:/ooooooooo+/:.`
                 ``````"##########
    )
}

pub fn open_mandriva() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                  ``````
            `-:/+++++++//:-.`
         .:+++oooo+/:.``   ``
      `:+ooooooo+:.  `-:/++++++/:.`
     -+oooooooo:` `-++o+/::::://+o+/-
   `/ooooooooo-  -+oo/.`        `-/oo+.
  `+ooooooooo.  :os/`              .+so:
  +sssssssss/  :ss/                 `+ss-
 :ssssssssss`  sss`                  .sso
 ossssssssss  `yyo                    sys
`sssssssssss` `yys                   `yys
`sssssssssss:  +yy/                  +yy:
 oyyyyyyyyyys. `oyy/`              `+yy+
 :yyyyyyyyyyyo. `+yhs:.         `./shy/
  oyyyyyyyyyyys:` .oyhys+:----/+syhy+. `
  `syyyyyyyyyyyyo-` .:osyhhhhhyys+:``.:`
   `oyyyyyyyyyyyyys+-`` `.----.```./oo.
     /yhhhhhhhhhhhhhhyso+//://+osyhy/`
      `/yhhhhhhhhhhhhhhhhhhhhhhhhy/`
        `:oyhhhhhhhhhhhhhhhhhhyo:`
            .:+syhhhhhhhhys+:-`
                 ``....``"##########
    )
}

pub fn open_stage() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}                 /(/
              .(((((((,
             /(((((((((/
           .(((((/,/(((((,
          *(((((*   ,(((((/
          (((((*      .*/((
         *((((/  (//(/*
         /((((*  ((((((((((,
      .  /((((*  (((((((((((((.
     ((. *((((/        ,((((((((
   ,(((/  (((((/     **   ,((((((*
  /(((((. .(((((/   //(((*  *(((((/
 .(((((,    ((/   .(((((/.   .(((((,
 /((((*        ,(((((((/      ,(((((
 /(((((((((((((((((((/.  /(((((((((/
 /(((((((((((((((((,   /(((((((((((/
     */(((((//*.      */((/(/(/*"##########
    )
}

pub fn open_wrt() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1} _______
|       |.-----.-----.-----.
|   -   ||  _  |  -__|     |
|_______||   __|_____|__|__|
         |__|
 ________        __
|  |  |  |.----.|  |_
|  |  |  ||   _||   _|
|________||__|  |____|"##########
    )
}

pub fn open_source_media_center() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}            -+shdmNNNNmdhs+-
        .+hMNho/:..``..:/ohNMh+.
      :hMdo.                .odMh:
    -dMy-                      -yMd-
   sMd-                          -dMs
  hMy       +.            .+       yMh
 yMy        dMs.        .sMd        yMy
:Mm         dMNMs`    `sMNMd        `mM:
yM+         dM//mNs``sNm//Md         +My
mM-         dM:  +NNNN+  :Md         -Mm
mM-         dM: `oNN+    :Md         -Mm
yM+         dM/+NNo`     :Md         +My
:Mm`        dMMNs`       :Md        `mM:
 yMy        dMs`         -ms        yMy
  hMy       +.                     yMh
   sMd-                          -dMs
    -dMy-                      -yMd-
      :hMdo.                .odMh:
        .+hMNho/:..``..:/ohNMh+.
            -+shdmNNNNmdhs+-"##########
    )
}

pub fn oracle() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}
      `-/+++++++++++++++++/-.`
   `/syyyyyyyyyyyyyyyyyyyyyyys/.
  :yyyyo/-...............-/oyyyy/
 /yyys-                     .oyyy+
.yyyy`                       `syyy-
:yyyo                         /yyy/
.yyyy`                       `syyy-
 /yyys.                     .oyyyo
  /yyyyo:-...............-:oyyyy/`
   `/syyyyyyyyyyyyyyyyyyyyyyys+.
     `.:/+ooooooooooooooo+/:.`"##########
    )
}

pub fn os_elbrus() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}   ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
   ██▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀██
   ██                       ██
   ██   ███████   ███████   ██
   ██   ██   ██   ██   ██   ██
   ██   ██   ██   ██   ██   ██
   ██   ██   ██   ██   ██   ██
   ██   ██   ██   ██   ██   ██
   ██   ██   ███████   ███████
   ██   ██                  ██
   ██   ██▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄██
   ██   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀██
   ██                       ██
   ███████████████████████████"##########
    )
}

pub fn pac_bsd() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}      :+sMs.
  `:ddNMd-                         -o--`
 -sMMMMh:                          `+N+``
 yMMMMMs`     .....-/-...           `mNh/
 yMMMMMmh+-`:sdmmmmmmMmmmmddy+-``./ddNMMm
 yNMMNMMMMNdyyNNMMMMMMMMMMMMMMMhyshNmMMMm
 :yMMMMMMMMMNdooNMMMMMMMMMMMMMMMMNmy:mMMd
  +MMMMMMMMMmy:sNMMMMMMMMMMMMMMMMMMMmshs-
  :hNMMMMMMN+-+MMMMMMMMMMMMMMMMMMMMMMMs.
 .omysmNNhy/+yNMMMMMMMMMMNMMMMMMMMMNdNNy-
 /hMM:::::/hNMMMMMMMMMMMm/-yNMMMMMMN.mMNh`
.hMMMMdhdMMMMMMMMMMMMMMmo  `sMMMMMMN mMMm-
:dMMMMMMMMMMMMMMMMMMMMMdo+  oMMMMMMN`smMNo`
/dMMMMMMMMMMMMMMMMMMMMMNd/` :yMMMMMN:-hMMM.
:dMMMMMMMMMMMMMMMMMMMMMNh`  oMMMMMMNo/dMNN`
:hMMMMMMMMMMMMMMMMMMMMMMNs--sMMMMMMMNNmy++`
 sNMMMMMMMMMMMMMMMMMMMMMMMmmNMMMMMMNho::o.
 :yMMMMMMMMMMMMMNho+sydNNNNNNNmysso/` -//
  /dMMMMMMMMMMMMMs-  ````````..``
   .oMMMMMMMMMMMMNs`               ./y:`
     +dNMMNMMMMMMMmy`          ``./ys.
      `/hMMMMMMMMMMMNo-``    `.+yy+-`
        `-/hmNMNMMMMMMmmddddhhy/-`
            `-+oooyMMMdsoo+/:."##########
    )
}

pub fn parabola_small() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}  __ __ __  _
.`_//_//_/ / `.
          /  .`
         / .`
        /.`
       /`"##########
    )
}

pub fn parabola() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}                          `.-.    `.
                   `.`  `:++.   `-+o+.
             `` `:+/. `:+/.   `-+oooo+
        ``-::-.:+/. `:+/.   `-+oooooo+
    `.-:///-  ..`   .-.   `-+oooooooo-
 `..-..`                 `+ooooooooo:
``                        :oooooooo/
                          `ooooooo:
                          `oooooo:
                          -oooo+.
                          +ooo/`
                         -ooo-
                        `+o/.
                        /+-
                       //`
                      -."##########
    )
}

pub fn pardus() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1} .smNdy+-    `.:/osyyso+:.`    -+ydmNs.
/Md- -/ymMdmNNdhso/::/oshdNNmdMmy/. :dM/
mN.     oMdyy- -y          `-dMo     .Nm
.mN+`  sMy hN+ -:             yMs  `+Nm.
 `yMMddMs.dy `+`               sMddMMy`
   +MMMo  .`  .                 oMMM+
   `NM/    `````.`    `.`````    +MN`
   yM+   `.-:yhomy    ymohy:-.`   +My
   yM:          yo    oy          :My
   +Ms         .N`    `N.      +h sM+
   `MN      -   -::::::-   : :o:+`NM`
    yM/    sh   -dMMMMd-   ho  +y+My
    .dNhsohMh-//: /mm/ ://-yMyoshNd`
      `-ommNMm+:/. oo ./:+mMNmmo:`
     `/o+.-somNh- :yy: -hNmos-.+o/`
    ./` .s/`s+sMdd+``+ddMs+s`/s. `/.
        : -y.  -hNmddmNy.  .y- :
         -+       `..`       +-"##########
    )
}

pub fn parrot() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}  `:oho/-`
`mMMMMMMMMMMMNmmdhy-
 dMMMMMMMMMMMMMMMMMMs`
 +MMsohNMMMMMMMMMMMMMm/
 .My   .+dMMMMMMMMMMMMMh.
  +       :NMMMMMMMMMMMMNo
           `yMMMMMMMMMMMMMm:
             /NMMMMMMMMMMMMMy`
              .hMMMMMMMMMMMMMN+
                  ``-NMMMMMMMMMd-
                     /MMMMMMMMMMMs`
                      mMMMMMMMsyNMN/
                      +MMMMMMMo  :sNh.
                      `NMMMMMMm     -o/
                       oMMMMMMM.
                       `NMMMMMM+
                        +MMd/NMh
                         mMm -mN`
                         /MM  `h:
                          dM`   .
                          :M-
                           d:
                           -+
                            -"##########
    )
}

pub fn parsix() -> String {
    let c1 = COLORS[3];
    let c2 = COLORS[1];
    let c3 = COLORS[7];
    let c4 = COLORS[8];
    format!(
        r##########"{c2}-/+/:.
               {c2}.syssssys.
       {c1}.--.    {c2}ssssssssso{c1}   ..--.
     :++++++:  {c2}+ssssssss+{c1} ./++/+++:
    /+++++++++.{c2}.yssooooy`{c1}-+///////o-
    /++++++++++.{c2}+soooos:{c1}:+////////+-
     :+++++////o-{c2}oooooo-{c1}+/////////-
      `-/++//++-{c4}.-----.-{c1}:+/////:-
  {c3}-://::--{c1}-:/:{c4}.--.````.--.{c1}:::-{c3}--::::::.
{c3}-/:::::::://:{c4}.:-`      `-:{c3}`:/:::::::--/-
{c3}/::::::::::/-{c4}--.        .-.{c3}-/://///::::/
{c3}-/:::::::::/:{c4}`:-.      .-:{c3}`:///////////-
 `{c3}-::::--{c1}.-://.{c4}---....---{c1}`:+/:-{c3}--::::-`
       {c1}-/+///+o/-{c4}.----.{c1}.:oo+++o+.
     {c1}-+/////+++o:{c2}syyyyy.{c1}o+++++++++:
    {c1}.+////+++++-{c2}+sssssy+{c1}.++++++++++\
    {c1}.+:/++++++.{c2}.yssssssy-{c1}`+++++++++:
     {c1}:/+++++-  {c2}+sssssssss  {c1}-++++++-
       {c1}`--`    {c2}+sssssssso    {c1}`--`
                {c2}+sssssy+`
                 {c2}`.::-`"##########
    )
}

pub fn pcbsd() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}                       ..
                        s.
                        +y
                        yN
                       -MN  `.
                      :NMs `m
                    .yMMm` `No
            `-/+++sdMMMNs+-`+Ms
        `:oo+-` .yMMMMy` `-+oNMh
      -oo-     +NMMMM/       oMMh-
    .s+` `    oMMMMM/     -  oMMMhy.
   +s`- ::   :MMMMMd     -o `mMMMy`s+
  y+  h .Ny+oNMMMMMN/    sh+NMMMMo  +y
 s+ .ds  -NMMMMMMMMMMNdhdNMMMMMMh`   +s
-h .NM`   `hMMMMMMMMMMMMMMNMMNy:      h-
y- hMN`     hMMmMMMMMMMMMNsdMNs.      -y
m` mMMy`    oMMNoNMMMMMMo`  sMMMo     `m
m` :NMMMdyydMMMMo+MdMMMs     sMMMd`   `m
h-  `+ymMMMMMMMM--M+hMMN/    +MMMMy   -h
:y     `.sMMMMM/ oMM+.yMMNddNMMMMMm   y:
 y:   `s  dMMN- .MMMM/ :MMMMMMMMMMh  :y
 `h:  `mdmMMM/  yMMMMs  sMMMMMMMMN- :h`
   so  -NMMMN   /mmd+  `dMMMMMMMm- os
    :y: `yMMM`       `+NMMMMMMNo`:y:
      /s+`.omy      /NMMMMMNh/.+s:
        .+oo:-.     /mdhs+::oo+.
            -/o+++++++++++/-"##########
    )
}

pub fn pc_linux_os() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}            mhhhyyyyhhhdN
        dyssyhhhhhhhhhhhssyhN
     Nysyhhyo/:-.....-/oyhhhssd
   Nsshhy+.              `/shhysm
  dohhy/                    -shhsy
 dohhs`                       /hhys
N+hho   {c2}+ssssss+-   .+syhys+   {c1}/hhsy
ohhh`   {c2}ymmo++hmm+`smmy/::+y`   {c1}shh+
+hho    {c2}ymm-  /mmy+mms          {c1}:hhod
/hh+    {c2}ymmhhdmmh.smm/          {c1}.hhsh
+hhs    {c2}ymm+::-`  /mmy`    `    {c1}/hh+m
yyhh-   {c2}ymm-       /dmdyosyd`  {c1}`yhh+
 ohhy`  {c2}://`         -/+++/-   {c1}ohhom
 N+hhy-                      `shhoh
   sshho.                  `+hhyom
    dsyhhs/.            `:ohhhoy
      dysyhhhso///://+syhhhssh
         dhyssyhhhhhhyssyyhN
              mddhdhdmN"##########
    )
}

pub fn pengwin() -> String {
    let c2 = COLORS[5];
    let c3 = COLORS[13];
    format!(
        r##########"{c3}                     ...`
{c3}                     `-///:-`
{c3}                       .+{c2}ssys{c3}/
{c3}                        +{c2}yyyyy{c3}o    {c2}
{c2}                        -yyyyyy:
{c2}           `.:/+ooo+/:` -yyyyyy+
{c2}         `:oyyyyyys+:-.`syyyyyy:
{c2}        .syyyyyyo-`   .oyyyyyyo
{c2}       `syyyyyy   `-+yyyyyyy/`
{c2}       /yyyyyy+ -/osyyyyyyo/.
{c2}       +yyyyyy-  `.-:::-.`
{c2}       .yyyyyy-
{c3}        :{c2}yyyyy{c3}o
{c3}         .+{c2}ooo{c3}+
{c3}           `.::/:."##########
    )
}

pub fn peppermint() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[15];
    format!(
        r##########"{c1}               PPPPPPPPPPPPPP
{c1}           PPPP{c2}MMMMMMM{c1}PPPPPPPPPPP
{c1}         PPPP{c2}MMMMMMMMMM{c1}PPPPPPPP{c2}MM{c1}PP
{c1}       PPPPPPPP{c2}MMMMMMM{c1}PPPPPPPP{c2}MMMMM{c1}PP
{c1}     PPPPPPPPPPPP{c2}MMMMMM{c1}PPPPPPP{c2}MMMMMMM{c1}PP
{c1}    PPPPPPPPPPPP{c2}MMMMMMM{c1}PPPP{c2}M{c1}P{c2}MMMMMMMMM{c1}PP
{c1}   PP{c2}MMMM{c1}PPPPPPPPPP{c2}MMM{c1}PPPPP{c2}MMMMMMM{c1}P{c2}MM{c1}PPPP
{c1}   P{c2}MMMMMMMMMM{c1}PPPPPP{c2}MM{c1}PPPPP{c2}MMMMMM{c1}PPPPPPPP
{c1}  P{c2}MMMMMMMMMMMM{c1}PPPPP{c2}MM{c1}PP{c2}M{c1}P{c2}MM{c1}P{c2}MM{c1}PPPPPPPPPPP
{c1}  P{c2}MMMMMMMMMMMMMMMM{c1}PP{c2}M{c1}P{c2}MMM{c1}PPPPPPPPPPPPPPPP
{c1}  P{c2}MMM{c1}PPPPPPPPPPPPPPPPPPPPPPPPPPPPPP{c2}MMMMM{c1}P
{c1}  PPPPPPPPPPPPPPPP{c2}MMM{c1}P{c2}M{c1}P{c2}MMMMMMMMMMMMMMMM{c1}PP
{c1}  PPPPPPPPPPP{c2}MM{c1}P{c2}MM{c1}PPPP{c2}MM{c1}PPPPP{c2}MMMMMMMMMMM{c1}PP
{c1}   PPPPPPPP{c2}MMMMMM{c1}PPPPP{c2}MM{c1}PPPPPP{c2}MMMMMMMMM{c1}PP
{c1}   PPPP{c2}MM{c1}P{c2}MMMMMMM{c1}PPPPPP{c2}MM{c1}PPPPPPPPPP{c2}MMMM{c1}PP
{c1}    PP{c2}MMMMMMMMM{c1}P{c2}M{c1}PPPP{c2}MMMMMM{c1}PPPPPPPPPPPPP
{c1}     PP{c2}MMMMMMM{c1}PPPPPPP{c2}MMMMMM{c1}PPPPPPPPPPPP
{c1}       PP{c2}MMMM{c1}PPPPPPPPP{c2}MMMMMMM{c1}PPPPPPPP
{c1}         PP{c2}MM{c1}PPPPPPPP{c2}MMMMMMMMMM{c1}PPPP
{c1}           PPPPPPPPPP{c2}MMMMMMMM{c1}PPPP
{c1}               PPPPPPPPPPPPPP"##########
    )
}

// pub fn Pisi() -> String {
// let c1 = Colors[12];
// let c2 = Colors[7];
// let c3 = Colors[6];
// let c4 = Colors[1];
// let c5 = Colors[8];
//   format!(
//     r##########"{c1}   \Fv/!-                      `:?lzC
// {c1}   Q!::=zFx!  {c2}`;v6WBCicl;`  {c1},vCC\!::#.
// {c1}  ,%:::,'` {c2}+#%@@FQ@@.   ,cF%i{c1}``-',::a?
// {c1}  +m:,'```{c2}}3,/@@Q\@@       "af-{c1} `-'"7f
//   =o'.` {c2}/m'   :Q@:Qg         ,kl{c1}  `.|o
//   :k` '{c2}$+      'Narm           >d,{c1}  ii
//    #`{c2}!p.        `C ,            'd+{c1} %'
// {c2}   !0m                           `6Kv
//    =a                              m+
//   !A     !\L|:            :|L\!     $:
//  .8`     Q''%Q#'        '#Q%''Q     `0-
//  :6      E|.6QQu        uQQ6.|E      p:
//   i{      \jts9?        ?9stj\      u\
//    |a`            -''.            `e>
//     ,m+     {c1}'^ !`{c2}s@@@@a{c1}'"`+`{c2}     >e'
//       !3|{c1}`|=>>r-  {c2}'U%:{c1}  '>>>=:`\3!
//        'xopE|      {c2}`'{c1}     `ledoz-
//     `;=>>+`{c2}`^llci/|==|/iclc;`{c1}'>>>>:
//    `^`+~          {c2}````{c1}          !!-^"##########
//   )
// }

// pub fn PNM_Linux() -> String {
//     let c1 = COLORS[33];
//     let c2 = COLORS[9];
//     let c3 = COLORS[15];
//     let c4 = COLORS[202];
//     format!(
//         r##########"{c1}
//                ``.---..` `--`
//             ``.---........-:.{c2}-::`{c1}
//            {c2}./::-{c1}........{c2}--::.````{c1}
//           {c2}.:://:::{c1}----{c2}::::-..{c1}
//           ..{c2}--:::::--::::++-{c1}.`
//   {c2}`-:-`{c1}   .-ohy+::{c2}-:::{c1}/sdmdd:.{c2}   `-:-
//    .-:::{c1}...{c3}sNNmds$y{c1}o/+{c3}sy+NN$m{c1}d+.`{c2}-:::-.
//      `.-:-{c1}./{c3}dN{c1}(){c3}yyooosd{c1}(){c3}$m{c1}dy{c2}-.::-.`{c1}
//       {c2}`.{c1}-...-{c3}+hNdyyyyyydmy{c1}:......{c2}`{c1}
//  ``..--.....-{c3}yNNm{c4}hssssh{c3}mmdo{c1}.........```
// `-:://:.....{c3}hNNNNN{c4}mddm{c3}NNNmds{c1}.....//::--`
//   ```.:-...{c3}oNNNNNNNNNNNNNNmd/{c1}...:-.```
//       .....{c3}hNNNNNNNNNNNNNNmds{c1}....`
//       --...{c3}hNNNNNNNNNNNNNNmdo{c1}.....
//       .:...{c3}/NNNNNNNNNNNNNNdd{c1}:....`
//        `-...{c3}+mNNNNNNNNNNNmh{c1}:...-.
//      {c4}.:+o+/:-{c1}:+oo+///++o+/:-{c4}:/+ooo/:.
//        {c4}+oo/:o-            +oooooso.`
//        {c4}.`   `             `/  .-//-"##########
//     )
// }

pub fn popos_small() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}______
\\   _ \\        __
 \\ \\ \\ \\      / /
  \\ \\_\\ \\    / /
   \\  ___\\  /_/
    \\ \\    _
   __\\_\\__(_)_
  (___________)`"##########
    )
}

pub fn pop_os() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}             /////////////
         /////////////////////
      ///////{c2}*767{c1}////////////////
    //////{c2}7676767676*{c1}//////////////
   /////{c2}76767{c1}//{c2}7676767{c1}//////////////
  /////{c2}767676{c1}///{c2}*76767{c1}///////////////
 ///////{c2}767676{c1}///{c2}76767{c1}.///{c2}7676*{c1}///////
/////////{c2}767676{c1}//{c2}76767{c1}///{c2}767676{c1}////////
//////////{c2}76767676767{c1}////{c2}76767{c1}/////////
///////////{c2}76767676{c1}//////{c2}7676{c1}//////////
////////////,{c2}7676{c1},///////{c2}767{c1}///////////
/////////////*{c2}7676{c1}///////{c2}76{c1}////////////
///////////////{c2}7676{c1}////////////////////
 ///////////////{c2}7676{c1}///{c2}767{c1}////////////
  //////////////////////{c2}'{c1}////////////
   //////{c2}.7676767676767676767,{c1}//////
    /////{c2}767676767676767676767{c1}/////
      ///////////////////////////
         /////////////////////
             /////////////"##########
    )
}

pub fn porteus() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}             `.-:::-.`
         -+ydmNNNNNNNmdy+-
      .+dNmdhs+//////+shdmdo.
    .smmy+-`             ./sdy:
  `omdo.    `.-/+osssso+/-` `+dy.
 `yms.   `:shmNmdhsoo++osyyo-``oh.
 hm/   .odNmds/.`    ``.....:::-+s
/m:  `+dNmy:`   `./oyhhhhyyooo++so
ys  `yNmy-    .+hmmho:-.`     ```
s:  yNm+`   .smNd+.
`` /Nm:    +dNd+`
   yN+   `smNy.
   dm    oNNy`
   hy   -mNm.
   +y   oNNo
   `y`  sNN:
    `:  +NN:
     `  .mNo
         /mm`
          /my`
           .sy`
             .+:
                `"##########
    )
}

pub fn postmarketos_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}        /\\
       /  \\
      /    \\
      \\__   \\
    /\\__ \\  _\\
   /   /  \\/ __
  /   / ____/  \\
 /    \\ \\       \\
/_____/ /________\\"##########
    )
}

pub fn post_market_os() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}                 /\\
                /  \\
               /    \\
              /      \\
             /        \\
            /          \\
            \\           \\
          /\\ \\____       \\
         /  \\____ \\       \\
        /       /  \\       \\
       /       /    \\    ___\\
      /       /      \\  / ____
     /       /        \\/ /    \\
    /       / __________/      \\
   /        \\ \\                 \\
  /          \\ \\                 \\
 /           / /                  \\
/___________/ /____________________\\"##########
    )
}

pub fn puff_os() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}
              _,..._,m,
            ,/'      '"";
           /             ".
         ,'mmmMMMMmm.      \
       _/-"^^^^^"""%#%mm,   ;
 ,m,_,'              "###)  ;,
(###%                 \#/  ;##mm.
 ^#/  __        ___    ;  (######)
  ;  //.\\     //.\\   ;   \####/
 _; (#\"//     \\"/#)  ;  ,/
@##\ \##/   =   `"=" ,;mm/
`\##>.____,...,____,<####@"##########
    )
}

// pub fn Proxmox() -> String {
//     let c1 = COLORS[7];
//     let c2 = COLORS[202];
//     format!(
//         r##########"{c1}         .://:`              `://:.
//        `hMMMMMMd/          /dMMMMMMh`
//         `sMMMMMMMd:      :mMMMMMMMs`
// {c2}`-/+oo+/:{c1}`.yMMMMMMMh-  -hMMMMMMMy.`{c2}:/+oo+/-`
// `:oooooooo/{c1}`-hMMMMMMMyyMMMMMMMh-`{c2}/oooooooo:`
//   `/oooooooo:{c1}`:mMMMMMMMMMMMMm:`{c2}:oooooooo/`
//     ./ooooooo+-{c1} +NMMMMMMMMN+ {c2}-+ooooooo/.
//       .+ooooooo+-{c1}`oNMMMMNo`{c2}-+ooooooo+.
//         -+ooooooo/.{c1}`sMMs`{c2}./ooooooo+-
//           :oooooooo/{c1}`..`{c2}/oooooooo:
//           :oooooooo/`{c1}..{c2}`/oooooooo:
//         -+ooooooo/.`{c1}sMMs{c2}`./ooooooo+-
//       .+ooooooo+-`{c1}oNMMMMNo{c2}`-+ooooooo+.
//     ./ooooooo+-{c1} +NMMMMMMMMN+ {c2}-+ooooooo/.
//   `/oooooooo:`{c1}:mMMMMMMMMMMMMm:{c2}`:oooooooo/`
// `:oooooooo/`{c1}-hMMMMMMMyyMMMMMMMh-{c2}`/oooooooo:`
// `-/+oo+/:`{c1}.yMMMMMMMh-  -hMMMMMMMy.{c2}`:/+oo+/-`
// {c1}        `sMMMMMMMm:      :dMMMMMMMs`
//        `hMMMMMMd/          /dMMMMMMh`
//          `://:`              `://:`"##########
//     )
// }

pub fn puppy() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}           `-/osyyyysosyhhhhhyys+-
  -ohmNNmh+/hMMMMMMMMNNNNd+dMMMMNM+
 yMMMMNNmmddo/NMMMNNNNNNNNNo+NNNNNy
.NNNNNNmmmddds:MMNNNNNNNNNNNh:mNNN/
-NNNdyyyhdmmmd`dNNNNNmmmmNNmdd/os/
.Nm+shddyooo+/smNNNNmmmmNh.   :mmd.
 NNNNy:`   ./hmmmmmmmNNNN:     hNMh
 NMN-    -++- +NNNNNNNNNNm+..-sMMMM-
.MMo    oNNNNo hNNNNNNNNmhdNNNMMMMM+
.MMs    /NNNN/ dNmhs+:-`  yMMMMMMMM+
 mMM+     .. `sNN+.      hMMMMhhMMM-
 +MMMmo:...:sNMMMMMms:` hMMMMm.hMMy
  yMMMMMMMMMMMNdMMMMMM::/+o+//dMMd`
   sMMMMMMMMMMN+:oyyo:sMMMNNMMMNy`
    :mMMMMMMMMMMMmddNMMMMMMMMmh/
      /dMMMMMMMMMMMMMMMMMMNdy/`
        .+hNMMMMMMMMMNmdhs/.
            .:/+ooo+/:-."##########
    )
}

pub fn pureos_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1} _____________
|  _________  |
| |         | |
| |         | |
| |_________| |
|_____________|"##########
    )
}

pub fn pure_os() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}dmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmd
dNm//////////////////////////////////mNd
dNd                                  dNd
dNd                                  dNd
dNd                                  dNd
dNd                                  dNd
dNd                                  dNd
dNd                                  dNd
dNd                                  dNd
dNd                                  dNd
dNm//////////////////////////////////mNd
dmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmd"##########
    )
}

pub fn qubes() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}               `..--..`
            `.----------.`
        `..----------------..`
     `.------------------------.``
 `..-------------....-------------..`
.::----------..``    ``..----------:+:
:////:----..`            `..---:/ossso
:///////:`                  `/osssssso
:///////:                    /ssssssso
:///////:                    /ssssssso
:///////:                    /ssssssso
:///////:                    /ssssssso
:///////:                    /ssssssso
:////////-`                .:sssssssso
:///////////-.`        `-/osssssssssso
`//////////////:-```.:+ssssssssssssso-
  .-://////////////sssssssssssssso/-`
     `.:///////////sssssssssssssso:.
         .-:///////ssssssssssssssssss/`
            `.:////ssss+/+ssssssssssss.
                `--//-    `-/osssso/."##########
    )
}

pub fn qubyt() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[5];
    let c3 = COLORS[0];
    format!(
        r##########"{c1}    ########################{c2}({c3}ooo
{c1}    ########################{c2}({c3}ooo
{c1}###{c2}({c3}ooo                  {c1}###{c2}({c3}ooo
{c1}###{c2}({c3}ooo                  {c1}###{c2}({c3}ooo
{c1}###{c2}({c3}ooo                  {c1}###{c2}({c3}ooo
{c1}###{c2}({c3}ooo                  {c1}###{c2}({c3}ooo
{c1}###{c2}({c3}ooo                  {c1}###{c2}({c3}ooo
{c1}###{c2}({c3}ooo                  {c1}###{c2}({c3}ooo
{c1}###{c2}({c3}ooo           {c1}##{c3}o    {c2}(((({c3}ooo
{c1}###{c2}({c3}ooo          o{c2}(({c1}###   {c3}oooooo
{c1}###{c2}({c3}ooo           oo{c2}(({c1}###{c3}o
{c1}###{c2}({c3}ooo             ooo{c2}(({c1}###
{c1}################{c2}({c3}oo    oo{c2}(((({c3}o
{c2}((((((((((((((((({c3}ooo     ooooo
  oooooooooooooooooo        o"##########
    )
}

pub fn quibian() -> String {
    let c1 = COLORS[3];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}            `.--::::::::--.`
        `.-:::-..``   ``..-::-.`
      .::::-`   .{c2}+{c1}:``       `.-::.`
    .::::.`    -::::::-`       `.::.
  `-:::-`    -:::::::::--..``     .::`
 `::::-     .{c2}oy{c1}:::::::---.```.:    `::`
 -::::  `.-:::::::::::-.```         `::
.::::.`-:::::::::::::.               `:.
-::::.:::::::::::::::                 -:
::::::::::::::::::::`                 `:
:::::::::::::::::::-                  `:
:::::::::::::::::::                   --
.:::::::::::::::::`                  `:`
`:::::::::::::::::                   -`
 .:::::::::::::::-                  -`
  `::::::::::::::-                `.`
    .::::::::::::-               ``
      `.--:::::-."##########
    )
}

pub fn radix() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[2];
    format!(
        r##########"{c2}                .:oyhdmNo
             `/yhyoosdms`
            -o+/ohmmho-
           ..`.:/:-`
     `.--:::-.``{c1}
  .+ydNMMMMMMNmhs:`
`omMMMMMMMMMMMMMMNh-
oNMMMNmddhhyyhhhddmy.
mMMMMNmmddhhysoo+/:-`
yMMMMMMMMMMMMMMMMNNh.
-dmmmmmNNMMMMMMMMMMs`
 -+oossyhmMMMMMMMMd-
 `sNMMMMMMMMMMMMMm:
  `yMMMMMMNmdhhhh:
   `sNMMMMMNmmho.
    `+mMMMMMMMy.
      .yNMMMm+`
       `:yd+."##########
    )
}

pub fn raspbian_small() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[1];
    format!(
        r##########"{c1}   ..    ,.
  :oo: .:oo:
  'o\\o o/o:
{c2} :: . :: . ::
:: :::  ::: ::
:'  '',.''  ':
 ::: :::: :::
 ':,  ''  ,:'
   ' ~::~ '"##########
    )
}

pub fn raspbian() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[1];
    format!(
        r##########"{c1}  `.::///+:/-.        --///+//-:``
 `+oooooooooooo:   `+oooooooooooo:
  /oooo++//ooooo:  ooooo+//+ooooo.
  `+ooooooo:-:oo-  +o+::/ooooooo:
   `:oooooooo+``    `.oooooooo+-
     `:++ooo/.        :+ooo+/.`
        {c2}...`  `.----.` ``..
     .::::-``:::::::::.`-:::-`
    -:::-`   .:::::::-`  `-:::-
   `::.  `.--.`  `` `.---.``.::`
       .::::::::`  -::::::::` `
 .::` .:::::::::- `::::::::::``::.
-:::` ::::::::::.  ::::::::::.`:::-
::::  -::::::::.   `-::::::::  ::::
-::-   .-:::-.``....``.-::-.   -::-
 .. ``       .::::::::.     `..`..
   -:::-`   -::::::::::`  .:::::`
   :::::::` -::::::::::` :::::::.
   .:::::::  -::::::::. ::::::::
    `-:::::`   ..--.`   ::::::.
      `...`  `...--..`  `...`
            .::::::::::
             `.-::::-`"##########
    )
}

pub fn reborn_os() -> String {
    let c1 = COLORS[2];
    let c3 = COLORS[8];
    format!(
        r##########"{c3}
        mMMMMMMMMM  MMMMMMMMMm
       NM                    MN
      MM  {c1}dddddddd  dddddddd  {c3}MN
     mM  {c1}dd                dd  {c3}MM
        {c1}dd  hhhhhh   hhhhh  dd
   {c3}mM      {c1}hh            hh      {c3}Mm
  NM  {c1}hd       {c3}mMMMMMMd       {c1}dh  {c3}MN
 NM  {c1}dd  hh   {c3}mMMMMMMMMm   {c1}hh  dd  {c3}MN
NM  {c1}dd  hh   {c3}mMMMMMMMMMMm   {c1}hh  dd  {c3}MN
 NM  {c1}dd  hh   {c3}mMMMMMMMMm   {c1}hh  dd  {c3}MN
  NM  {c1}hd       {c3}mMMMMMMm       {c1}dh  {c3}MN
   mM      {c1}hh            hh      {c3}Mm
        {c1}dd  hhhhhh  hhhhhh  dd
     {c3}MM  {c1}dd                dd  {c3}MM
      MM  {c1}dddddddd  dddddddd  {c3}MN
       NM                    MN
        mMMMMMMMMM  MMMMMMMMMm"##########
    )
}

pub fn red_star() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}                    ..
                  .oK0l
                 :0KKKKd.
               .xKO0KKKKd
              ,Od' .d0000l
             .c;.   .'''...           ..'.
.,:cloddxxxkkkkOOOOkkkkkkkkxxxxxxxxxkkkx:
;kOOOOOOOkxOkc'...',;;;;,,,'',;;:cllc:,.
 .okkkkd,.lko  .......',;:cllc:;,,'''''.
   .cdo. :xd' cd:.  ..';'',,,'',,;;;,'.
      . .ddl.;doooc'..;oc;'..';::;,'.
        coo;.oooolllllllcccc:'.  .
       .ool''lllllccccccc:::::;.
       ;lll. .':cccc:::::::;;;;'
       :lcc:'',..';::::;;;;;;;,,.
       :cccc::::;...';;;;;,,,,,,.
       ,::::::;;;,'.  ..',,,,'''.
        ........          ......"##########
    )
}

pub fn redcore() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}                 RRRRRRRRR
               RRRRRRRRRRRRR
        RRRRRRRRRR      RRRRR
   RRRRRRRRRRRRRRRRRRRRRRRRRRR
 RRRRRRR  RRR         RRR RRRRRRRR
RRRRR    RR                 RRRRRRRRR
RRRR    RR     RRRRRRRR      RR RRRRRR
RRRR   R    RRRRRRRRRRRRRR   RR   RRRRR
RRRR   R  RRRRRRRRRRRRRRRRRR  R   RRRRR
RRRR     RRRRRRRRRRRRRRRRRRR  R   RRRR
 RRR     RRRRRRRRRRRRRRRRRRRR R   RRRR
  RRR    RRRRRRRRRRRRRRRRRRRR    RRRR
    RR   RRRRRRRRRRRRRRRRRRR    RRR
     RR   RRRRRRRRRRRRRRRRR    RRR
       RR   RRRRRRRRRRRRRR   RR
         R       RRRR      RR"##########
    )
}

pub fn redhat_old() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}             `.-..........`
            `////////::.`-/.
            -: ....-////////.
            //:-::///////////`
     `--::: `-://////////////:
     //////-    ``.-:///////// .`
     `://////:-.`    :///////::///:`
       .-/////////:---/////////////:
          .-://////////////////////.
{c2}         yMN+`.-{c1}::///////////////-`
{c2}      .-`:NMMNMs`  `..-------..`
       MN+/mMMMMMhoooyysshsss
MMM    MMMMMMMMMMMMMMyyddMMM+
 MMMM   MMMMMMMMMMMMMNdyNMMh`     hyhMMM
  MMMMMMMMMMMMMMMMyoNNNMMM+.   MMMMMMMM
   MMNMMMNNMMMMMNM+ mhsMNyyyyMNMMMMsMM"##########
    )
}

pub fn redhat() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}           .MMM..:MMMMMMM
          MMMMMMMMMMMMMMMMMM
          MMMMMMMMMMMMMMMMMMMM.
         MMMMMMMMMMMMMMMMMMMMMM
        ,MMMMMMMMMMMMMMMMMMMMMM:
        MMMMMMMMMMMMMMMMMMMMMMMM
  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM
 MMMMMM    `MMMMMMMMMMMMMMMMMMMM.
MMMMMMMM      MMMMMMMMMMMMMMMMMM .
MMMMMMMMM.       `MMMMMMMMMMMMM' MM.
MMMMMMMMMMM.                     MMMM
`MMMMMMMMMMMMM.                 ,MMMMM.
 `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM.
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:
         MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
            `MMMMMMMMMMMMMMMMMMMMMMMM:
                ``MMMMMMMMMMMMMMMMM'"##########
    )
}

pub fn refracted_devuan() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                             A
                            VW
                           VVW\\
                         .yWWW\\
 ,;,,u,;yy;;v;uyyyyyyy  ,WWWWW^
    *WWWWWWWWWWWWWWWW/  $VWWWWw      ,
        ^*%WWWWWWVWWX  $WWWW**    ,yy
        ,    "**WWW/' **'   ,yy/WWW*`
       &WWWWwy    `*`  <,ywWW%VWWW*
     yWWWWWWWWWW*    .,   "**WW%W
   ,&WWWWWM*"`  ,y/  &WWWww   ^*
  XWWX*^   ,yWWWW09 .WWWWWWWWwy,
 *`        &WWWWWM  WWWWWWWWWWWWWww,
           (WWWWW` /#####WWW***********
           ^WWWW
            VWW
            Wh.
            V/"##########
    )
}

pub fn regata() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[1];
    let c3 = COLORS[4];
    let c4 = COLORS[5];
    let c5 = COLORS[3];
    let c6 = COLORS[2];
    format!(
        r##########"{c1}            ddhso+++++osydd
        dho/.`hh{c2}.:/+/:.{c1}hhh`:+yd
      do-hhhhhh{c2}/sssssss+`{c1}hhhhh./yd
    h/`hhhhhhh{c2}-sssssssss:{c1}hhhhhhhh-yd
  do`hhhhhhhhh{c2}`ossssssso.{c1}hhhhhhhhhh/d
 d/hhhhhhhhhhhh{c2}`/ossso/.{c1}hhhhhhhhhhhh.h
 /hhhhhhhhhhhh{c3}`-/osyso/-`{c1}hhhhhhhhhhhh.h
shh{c4}-/ooo+-{c1}hhh{c3}:syyso+osyys/`{c1}hhh{c5}`+oo`{c1}hhh/
h{c4}`ohhhhhhho`{c3}+yyo.{c1}hhhhh{c3}.+yyo`{c5}.sssssss.{c1}h`h
s{c4}:hhhhhhhhho{c3}yys`{c1}hhhhhhh{c3}.oyy/{c5}ossssssso-{c1}hs
s{c4}.yhhhhhhhy/{c3}yys`{c1}hhhhhhh{c3}.oyy/{c5}ossssssso-{c1}hs
hh{c4}./syyys+.{c1} {c3}+yy+.{c1}hhhhh{c3}.+yyo`{c5}.ossssso/{c1}h`h
shhh{c4}``.`{c1}hhh{c3}`/syyso++oyys/`{c1}hhh{c5}`+++-`{c1}hh:h
d/hhhhhhhhhhhh{c3}`-/osyso+-`{c1}hhhhhhhhhhhh.h
 d/hhhhhhhhhhhh{c6}`/ossso/.{c1}hhhhhhhhhhhh.h
  do`hhhhhhhhh{c6}`ossssssso.{c1}hhhhhhhhhh:h
    h/`hhhhhhh{c6}-sssssssss:{c1}hhhhhhhh-yd
      h+.hhhhhh{c6}+sssssss+{c1}hhhhhh`/yd
        dho:.hhh{c6}.:+++/.{c1}hhh`-+yd
            ddhso+++++osyhd"##########
    )
}

pub fn regolith() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}
                 ``....```
            `.:/++++++/::-.`
          -/+++++++:.`
        -++++++++:`
      `/++++++++-
     `/++++++++.                    -/+/
     /++++++++/             ``   .:+++:.
    -+++++++++/          ./++++:+++/-`
    :+++++++++/         `+++++++/-`
    :++++++++++`      .-/+++++++`
   `:++++++++++/``.-/++++:-:::-`      `
 `:+++++++++++++++++/:.`            ./`
:++/-:+++++++++/:-..              -/+.
+++++++++/::-...:/+++/-..````..-/+++.
`......``.::/+++++++++++++++++++++/.
         -/+++++++++++++++++++++/.
           .:/+++++++++++++++/-`
              `.-:://////:-."##########
    )
}

pub fn rocky_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}    `-/+++++++++/-.`
 `-+++++++++++++++++-`
.+++++++++++++++++++++.
-+++++++++++++++++++++++.
+++++++++++++++/-/+++++++
+++++++++++++/.   ./+++++
+++++++++++:.       ./+++
+++++++++:`   `:/:`   .:/
-++++++:`   .:+++++:`
 .+++-`   ./+++++++++:`
  `-`   ./+++++++++++-
       -+++++++++:-.`"##########
    )
}

// pub fn rocky() -> String {
// let c1 = Colors[35];
//   format!(
//     r##########"{c1}          __wgliliiligw_,
//        _williiiiiiliilililw,
//      _%iiiiiilililiiiiiiiiiii_
//    .Qliiiililiiiiiiililililiilm.
//   _iiiiiliiiiiililiiiiiiiiiiliil,
//  .lililiiilililiiiilililililiiiii,
// _liiiiiiliiiiiiiliiiiiF{iiiiiilili,
// jliililiiilililiiili@`  ~ililiiiiiL
// iiiliiiiliiiiiiili>`      ~liililii
// liliiiliiilililii`         -9liiiil
// iiiiiliiliiiiii~             "4lili
// 4ililiiiiilil~|      -w,       )4lf
// -liiiiililiF'       _liig,       )'
//  )iiiliii@`       _QIililig,
//   )iiii>`       .Qliliiiililw
//    )<>~       .mliiiiiliiiiiil,
//             _gllilililiililii~
//            giliiiiiiiiiiiiT`
//           -^~$ililili@~~'"##########
//   )
// }

pub fn rosa() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}           ROSAROSAROSAROSAR
        ROSA               AROS
      ROS   SAROSAROSAROSAR   AROS
    RO   ROSAROSAROSAROSAROSAR   RO
  ARO  AROSAROSAROSARO      AROS  ROS
 ARO  ROSAROS         OSAR   ROSA  ROS
 RO  AROSA   ROSAROSAROSA    ROSAR  RO
RO  ROSAR  ROSAROSAROSAR  R  ROSARO  RO
RO  ROSA  AROSAROSAROSA  AR  ROSARO  AR
RO AROS  ROSAROSAROSA   ROS  AROSARO AR
RO AROS  ROSAROSARO   ROSARO  ROSARO AR
RO  ROS  AROSAROS   ROSAROSA AROSAR  AR
RO  ROSA  ROS     ROSAROSAR  ROSARO  RO
 RO  ROS     AROSAROSAROSA  ROSARO  AR
 ARO  ROSA   ROSAROSAROS   AROSAR  ARO
  ARO  OROSA      R      ROSAROS  ROS
    RO   AROSAROS   AROSAROSAR   RO
     AROS   AROSAROSAROSARO   AROS
        ROSA               SARO
           ROSAROSAROSAROSAR"##########
    )
}

pub fn sabotage() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2} .|'''.|      |     '||''|.    ..|''||
 ||..  '     |||     ||   ||  .|'    ||
  ''|||.    |  ||    ||'''|.  ||      ||
.     '||  .''''|.   ||    || '|.     ||
|'....|'  .|.  .||. .||...|'   ''|...|'

|''||''|     |      ..|'''.|  '||''''|
   ||       |||    .|'     '   ||  .
   ||      |  ||   ||    ....  ||''|
   ||     .''''|.  '|.    ||   ||
  .||.   .|.  .||.  ''|...'|  .||.....|"##########
    )
}

pub fn sabayon() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}            ...........
         ..             ..
      ..                   ..
    ..           {c2}o           {c1}..
  ..            {c2}:W'            {c1}..
 ..             {c2}.d.             {c1}..
:.             {c2}.KNO              {c1}.:
:.             {c2}cNNN.             {c1}.:
:              {c2}dXXX,              {c1}:
:   {c2}.          dXXX,       .cd,   {c1}:
:   {c2}'kc ..     dKKK.    ,ll;:'    {c1}:
:     {c2}.xkkxc;..dkkkc',cxkkl       {c1}:
:.     {c2}.,cdddddddddddddo:.       {c1}.:
 ..         {c2}:lllllll:           {c1}..
   ..         {c2}',,,,,          {c1}..
     ..                     ..
        ..               ..
          ..............."##########
    )
}

pub fn sailfish() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                 _a@b
              _#b (b
            _@@   @_         _,
          _#^@ _#*^^*gg,aa@^^
          #- @@^  _a@^^
          @_  *g#b
          ^@_   ^@_
            ^@_   @
             @(b (b
            #b(b#^
          _@_#@^
       _a@a*^
   ,a@*^"##########
    )
}

pub fn salent_os() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[1];
    let c3 = COLORS[3];
    let c4 = COLORS[7];
    format!(
        r##########"{c1}                 ``..``
        .-:+oshdNMMMMMMNdhyo+:-.`
  -oydmMMMMMMMMMMMMMMMMMMMMMMMMMMNdhs/
{c4} +hdddm{c1}NMMMMMMMMMMMMMMMMMMMMMMMMN{c4}mdddh+`
{c2}`MMMMMN{c4}mdddddm{c1}MMMMMMMMMMMM{c4}mdddddm{c3}NMMMMM-
{c2} mMMMMMMMMMMMN{c4}ddddhyyhhddd{c3}NMMMMMMMMMMMM`
{c2} dMMMMMMMMMMMMMMMMM{c4}oo{c3}MMMMMMMMMMMMMMMMMN`
{c2} yMMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMMd
{c2} +MMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMMy
{c2} :MMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMMo
{c2} .MMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMM/
{c2} `NMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMM-
{c2}  mMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMN`
{c2}  hMMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMm
{c2}  /MMMMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMMMMy
{c2}   .+hMMMMMMMMMMMMM{c4}hh{c3}MMMMMMMMMMMMMms:
{c2}      `:smMMMMMMMMM{c4}hh{c3}MMMMMMMMMNh+.
{c2}          .+hMMMMMM{c4}hh{c3}MMMMMMdo:
{c2}             `:smMM{c4}yy{c3}MMNy/`
                 {c2}.- {c4}`{c3}:."##########
    )
}

pub fn scientific() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    let c3 = COLORS[1];
    format!(
        r##########"{c1}                 =/;;/-
                +:    //
               /;      /;
              -X        H.
.//;;;:;;-,   X=        :+   .-;:=;:;#;.
M-       ,=;;;#:,      ,:#;;:=,       ,@
:#           :#.=/++++/=.$=           #=
 ,#;         #/:+/;,,/++:+/         ;+.
   ,+/.    ,;@+,        ,#H;,    ,/+,
      ;+;;/= @.  {c3}.H{c2}#{c3}#X   {c1}-X :///+;
      ;+=;;;.@,  {c2}.X{c3}M{c2}@$.  {c1}=X.//;=#/.
   ,;:      :@#=        =$H:     .+#-
 ,#=         #;-///==///-//         =#,
;+           :#-;;;:;;;;-X-           +:
@-      .-;;;;M-        =M/;;;-.      -X
 :;;::;;-.    #-        :+    ,-;;-;:==
              ,X        H.
               ;/      #=
                //    +;
                 '////'"##########
    )
}

pub fn septor() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    let c3 = COLORS[4];
    format!(
        r##########"{c1}ssssssssssssssssssssssssssssssssssssssss
ssssssssssssssssssssssssssssssssssssssss
ssssssssssssssssssssssssssssssssssssssss
ssssssssssssssssssssssssssssssssssssssss
ssssssssss{c2};okOOOOOOOOOOOOOOko;{c1}ssssssssss
sssssssss{c2}oNWWWWWWWWWWWWWWWWWWNo{c1}sssssssss
ssssssss{c2}:WWWWWWWWWWWWWWWWWWWWWW:{c1}ssssssss
ssssssss{c2}lWWWWWk{c1}ssssssssss{c2}lddddd:{c1}ssssssss
ssssssss{c2}cWWWWWNKKKKKKKKKKKKOx:{c1}ssssssssss
{c3}yy{c1}sssssss{c2}OWWWWWWWWWWWWWWWWWWWWx{c1}sssssss{c3}yy
yyyyyyyyyy{c2}:kKNNNNNNNNNNNNWWWWWW:{c3}yyyyyyyy
yyyyyyyy{c2}sccccc;{c3}yyyyyyyyyy{c2}kWWWWW:{c3}yyyyyyyy
yyyyyyyy{c2}:WWWWWWNNNNNNNNNNWWWWWW;{c3}yyyyyyyy
yyyyyyyy{c2}.dWWWWWWWWWWWWWWWWWWWNd{c3}yyyyyyyyy
yyyyyyyyyy{c2}sdO0KKKKKKKKKKKK0Od;{c3}yyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"##########
    )
}

pub fn serene() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}              __---''''''---__
          .                      .
        :                          :
      -                       _______----_-
     s               __----'''     __----
 __h_            _-'           _-'     h
 '-._''--.._    ;           _-'         y
  :  ''-._  '-._/        _-'             :
  y       ':_       _--''                y
  m    .--'' '-._.;'                     m
  m   :        :                         m
  y    '.._     '-__                     y
  :        '--._    '''----___           :
   y            '--._         ''-- _    y
    h                '--._          :  h
     s                  __';         vs
      -         __..--''             -
        :_..--''                   :
          .                     _ .
            `''---______---''-``"##########
    )
}

pub fn shark_linux() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}                              `:shd/
                          `:yNMMMMs
                       `-smMMMMMMN.
                     .+dNMMMMMMMMs
                   .smNNMMMMMMMMm`
                 .sNNNNNNNMMMMMM/
               `omNNNNNNNMMMMMMm
              /dNNNNNNNNMMMMMMM+
            .yNNNNNNNNNMMMMMMMN`
           +mNNNNNNNNNMMMMMMMMh
         .hNNNNNNNNNNMMMMMMMMMs
        +mMNNNNNNNNMMMMMMMMMMMs
      .hNMMNNNNMMMMMMMMMMMMMMMd
    .oNNNNNNNNNNMMMMMMMMMMMMMMMo
 `:+syyssoo++++ooooossssssssssso:"##########
    )
}

// pub fn Siduction() -> String {
// let c1 = Colors[4];
// let c2 = Colors[4];
//   format!(
//     r##########"{c1}                _aass,
//                jQh: =$w
//                QWmwawQW
//                )$QQQQ@(   ..
//          _a_a.   ~??^  syDY?Sa,
//        _mW>-<$c       jWmi  imm.
//        ]QQwayQE       4QQmgwmQQ`
//         ?WWQWP'       -9QQQQQ@'._aas,
//  _a%is.        .adYYs,. -"?!` aQB*~^3$c
// _Qh;.nm       .QWc. {QL      ]QQp;..vmQ/
// "QQmmQ@       -QQQggmQP      ]QQWmggmQQ(
//  -???"         "$WQQQY`  __,  ?QQQQQQW!
//         _yZ!?q,   -   .yWY!!Sw, "???^
//        .QQa_=qQ       mQm>..vmm
//         $QQWQQP       $QQQgmQQ@
//          "???"   _aa, -9WWQQWY`
//                _mB>~)$a  -~~
//                mQms_vmQ.
//                ]WQQQQQP
//                 -?T??""##########
//   )
// }

pub fn slackware_small() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}   ________
  /  ______|
  | |______
  \\______  \\
   ______| |
| |________/
|____________"##########
    )
}

pub fn slackware() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}                  :::::::
            :::::::::::::::::::
         :::::::::::::::::::::::::
       ::::::::{c2}cllcccccllllllll{c1}::::::
    :::::::::{c2}lc               dc{c1}:::::::
   ::::::::{c2}cl   clllccllll    oc{c1}:::::::::
  :::::::::{c2}o   lc{c1}::::::::{c2}co   oc{c1}::::::::::
 ::::::::::{c2}o    cccclc{c1}:::::{c2}clcc{c1}::::::::::::
 :::::::::::{c2}lc        cclccclc{c1}:::::::::::::
::::::::::::::{c2}lcclcc          lc{c1}::::::::::::
::::::::::{c2}cclcc{c1}:::::{c2}lccclc     oc{c1}:::::::::::
::::::::::{c2}o    l{c1}::::::::::{c2}l    lc{c1}:::::::::::
 :::::{c2}cll{c1}:{c2}o     clcllcccll     o{c1}:::::::::::
 :::::{c2}occ{c1}:{c2}o                  clc{c1}:::::::::::
  ::::{c2}ocl{c1}:{c2}ccslclccclclccclclc{c1}:::::::::::::
   :::{c2}oclcccccccccccccllllllllllllll{c1}:::::
    ::{c2}lcc1lcccccccccccccccccccccccco{c1}::::
      ::::::::::::::::::::::::::::::::
        ::::::::::::::::::::::::::::
           ::::::::::::::::::::::
                ::::::::::::"##########
    )
}

pub fn sli_taz() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}        @    @(               @
      @@   @@                  @    @/
     @@   @@                   @@   @@
    @@  %@@                     @@   @@
   @@  %@@@       @@@@@.       @@@@  @@
  @@@    @@@@    @@@@@@@    &@@@    @@@
   @@@@@@@ %@@@@@@@@@@@@ &@@@% @@@@@@@/
       ,@@@@@@@@@@@@@@@@@@@@@@@@@
  .@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@/
@@@@@@.  @@@@@@@@@@@@@@@@@@@@@  /@@@@@@
@@    @@@@@  @@@@@@@@@@@@,  @@@@@   @@@
@@ @@@@.    @@@@@@@@@@@@@%    #@@@@ @@.
@@ ,@@      @@@@@@@@@@@@@      @@@  @@
@   @@.     @@@@@@@@@@@@@     @@@  *@
@    @@     @@@@@@@@@@@@      @@   @
      @      @@@@@@@@@.     #@
       @      ,@@@@@       @"##########
    )
}

pub fn smart_os() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyys             oyyyyyyyyyyyyyyyy
yyyys  yyyyyyyyy  oyyyyyyyyyyyyyyyy
yyyys  yyyyyyyyy  oyyyyyyyyyyyyyyyy
yyyys  yyyyyyyyy  oyyyyyyyyyyyyyyyy
yyyys  yyyyyyyyy  oyyyyyyyyyyyyyyyy
yyyys  yyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyy                         syyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyy  syyyy
yyyyyyyyyyyyyyyy  syyyyyyyyy  syyyy
yyyyyyyyyyyyyyyy  oyyyyyyyyy  syyyy
yyyyyyyyyyyyyyyy  oyyyyyyyyy  syyyy
yyyyyyyyyyyyyyyy  syyyyyyyyy  syyyy
yyyyyyyyyyyyyyyy              yyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"##########
    )
}

// pub fn SkiffOS() -> String {
// let c1 = Colors[4];
// let c2 = Colors[7];
//   format!(
//     r##########"{c2}
//              ,@@@@@@@@@@@w,_
//   {c2}====~~~,,.{c2}A@@@@@@@@@@@@@@@@@W,_
//   {c1}`||||||||||||||L{{c2}"@$@@@@@@@@B"
//    {c1}`|||||||||||||||||||||L{{c2}"$D
//      {c2}@@@@@@@@@@@@@@@@@@@@@{c1}_||||}==,
//       {c2}*@@@@@@@@@@@@@@@@@@@@@@@@@p{c1}||||==,
//         {c1}`'||LLL{{""{c2}@$B@@@@@@@@@@@@@@@p{c1}||
//             {c1}`~=|||||||||||L"{c2}$@@@@@@@@@@@
//                    {c1}````'"""""""{c2}'"""""""""##########
//   )
// }

pub fn solus() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}            -```````````
          `-+/------------.`
       .---:mNo---------------.
     .-----yMMMy:---------------.
   `------oMMMMMm/----------------`
  .------/MMMMMMMN+----------------.
 .------/NMMMMMMMMm-+/--------------.
`------/NMMMMMMMMMN-:mh/-------------`
.-----/NMMMMMMMMMMM:-+MMd//oso/:-----.
-----/NMMMMMMMMMMMM+--mMMMh::smMmyo:--
----+NMMMMMMMMMMMMMo--yMMMMNo-:yMMMMd/.
.--oMMMMMMMMMMMMMMMy--yMMMMMMh:-yMMMy-`
`-sMMMMMMMMMMMMMMMMh--dMMMMMMMd:/Ny+y.
`-/+osyhhdmmNNMMMMMm-/MMMMMMMmh+/ohm+
  .------------:://+-/++++++{c1}oshddys:
   -hhhhyyyyyyyyyyyhhhhddddhysssso-
    `:ossssssyysssssssssssssssso:`
      `:+ssssssssssssssssssss+-
         `-/+ssssssssssso+/-`
              `.-----..`"##########
    )
}

pub fn source_mage() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}       :ymNMNho.
.+sdmNMMMMMMMMMMy`
.-::/yMMMMMMMMMMMm-
      sMMMMMMMMMMMm/
     /NMMMMMMMMMMMMMm:
    .MMMMMMMMMMMMMMMMM:
    `MMMMMMMMMMMMMMMMMN.
     NMMMMMMMMMMMMMMMMMd
     mMMMMMMMMMMMMMMMMMMo
     hhMMMMMMMMMMMMMMMMMM.
     .`/MMMMMMMMMMMMMMMMMs
        :mMMMMMMMMMMMMMMMN`
         `sMMMMMMMMMMMMMMM+
           /NMMMMMMMMMMMMMN`
             oMMMMMMMMMMMMM+
          ./sd.-hMMMMMMMMmmN`
      ./+oyyyh- `MMMMMMMMMmNh
                 sMMMMMMMMMmmo
                 `NMMMMMMMMMd:
                  -dMMMMMMMMMo
                    -shmNMMms."##########
    )
}

pub fn sparky() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}
           .            `-:-`
          .o`       .-///-`
         `oo`    .:/++:.
         os+`  -/+++:` ``.........```
        /ys+`./+++/-.-::::::----......``
       `syyo`++o+--::::-::/+++/-``
       -yyy+.+o+`:/:-:sdmmmmmmmmdy+-`
::-`   :yyy/-oo.-+/`ymho++++++oyhdmdy/`
`/yy+-`.syyo`+o..o--h..osyhhddhs+//osyy/`
  -ydhs+-oyy/.+o.-: ` `  :/::+ydhy+```-os-
   .sdddy::syo--/:.     `.:dy+-ohhho    ./:
     :yddds/:+oo+//:-`- /+ +hy+.shhy:     ``
      `:ydmmdysooooooo-.ss`/yss--oyyo
        `./ossyyyyo+:-/oo:.osso- .oys
       ``..-------::////.-oooo/   :so
    `...----::::::::--.`/oooo:    .o:
           ```````     ++o+:`     `:`
                     ./+/-`        `
                   `-:-.
                   ``"##########
    )
}

pub fn star() -> String {
    let c1 = COLORS[7];
    format!(
        r##########"{c1}                   ./
                  `yy-
                 `y.`y`
    ``           s-  .y            `
    +h//:..`    +/    /o    ``..:/so
     /o``.-::/:/+      o/://::-.`+o`
      :s`     `.        .`     `s/
       .y.                    .s-
        `y-                  :s`
      .-//.                  /+:.
   .:/:.                       .:/:.
-+o:.                             .:+:.
-///++///:::`              .-::::///+so-
       ``..o/              d-....```
           s.     `/.      d
           h    .+o-+o-    h.
           h  -o/`   `/o:  s:
          -s/o:`       `:o/+/
          /s-             -yo"##########
    )
}

pub fn steam_os() -> String {
    let c1 = COLORS[5];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}              .,,,,.
        .,'onNMMMMMNNnn',.
     .'oNMANKMMMMMMMMMMMNNn'.
   .'ANMMMMMMMXKNNWWWPFFWNNMNn.
  ;NNMMMMMMMMMMNWW'' ,.., 'WMMM,
 ;NMMMMV+##+VNWWW' .+;'':+, 'WMW,
,VNNWP+{c2}######{c1}+WW,  {c2}+:    {c1}:+, +MMM,
'{c2}+#############,   +.    ,+' {c1}+NMMM
{c2}  '*#########*'     '*,,*' {c1}.+NMMMM.
{c2}     `'*###*'          ,.,;###{c1}+WNM,
{c2}         .,;;,      .;##########{c1}+W
{c2},',.         ';  ,+##############'
 '###+. :,. .,; ,###############'
  '####.. `'' .,###############'
    '#####+++################'
      '*##################*'
         ''*##########*''
              ''''''"##########
    )
}

pub fn sunos_small() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}       .   .;   .
   .   :;  ::  ;:   .
   .;. ..      .. .;.
..  ..             ..  ..
 .;,                 ,;."##########
    )
}

pub fn open_suse_leap() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                 `-++:`
               ./oooooo/-
            `:oooooooooooo:.
          -+oooooooooooooooo+-`
       ./oooooooooooooooooooooo/-
      :oooooooooooooooooooooooooo:
    `  `-+oooooooooooooooooooo/-   `
 `:oo/-   .:ooooooooooooooo+:`  `-+oo/.
`/oooooo:.   -/oooooooooo/.   ./oooooo/.
  `:+ooooo+-`  `:+oooo+-   `:oooooo+:`
     .:oooooo/.   .::`   -+oooooo/.
        -/oooooo:.    ./oooooo+-
          `:+ooooo+-:+oooooo:`
             ./oooooooooo/.
                -/oooo+:`
                  `:/."##########
    )
}

pub fn t2() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[4];
    format!(
        r##########"{c2}
TTTTTTTTTT
    tt   {c1}222{c2}
    tt  {c1}2   2{c2}
    tt     {c1}2{c2}
    tt    {c1}2{c2}
    tt  {c1}22222{c2}"##########
    )
}

pub fn open_suse_tumbleweed() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}                                     ......
     .,cdxxxoc,.               .:kKMMMNWMMMNk:.
    cKMMN0OOOKWMMXo. ;        ;0MWk:.      .:OMMk.
  ;WMK;.       .lKMMNM,     :NMK,             .OMW;
 cMW;            'WMMMN   ,XMK,                 oMM'
.MMc               ..;l. xMN:                    KM0
'MM.                   'NMO                      oMM
.MM,                 .kMMl                       xMN
 KM0               .kMM0. .dl:,..               .WMd
 .XM0.           ,OMMK,    OMMMK.              .XMK
   oWMO:.    .;xNMMk,       NNNMKl.          .xWMx
     :ONMMNXMMMKx;          .  ,xNMWKkxllox0NMWk,
         .....                    .:dOOXXKOxl,"##########
    )
}

pub fn opensuse_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}  _______
__|   __ \\
     / .\\ \\
     \\__/ |
   _______|
   \\_______
__________/"##########
    )
}

pub fn open_suse() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}           .;ldkO0000Okdl;.
       .;d00xl:^''''''^:ok00d;.
     .d00l'                'o00d.
   .d0Kd'{c1}  Okxol:;,.          {c2}:O0d.
  .OK{c1}KKK0kOKKKKKKKKKKOxo:,      {c2}lKO.
 ,0K{c1}KKKKKKKKKKKKKKK0P^{c2},,,{c1}^dx:{c2}    ;00,
.OK{c1}KKKKKKKKKKKKKKKk'{c2}.oOPPb.{c1}'0k.{c2}   cKO.
:KK{c1}KKKKKKKKKKKKKKK: {c2}kKx..dd {c1}lKd{c2}   'OK:
dKK{c1}KKKKKKKKKOx0KKKd {c2}^0KKKO' {c1}kKKc{c2}   dKd
dKK{c1}KKKKKKKKKK;.;oOKx,..{c2}^{c1}..;kKKK0.{c2}  dKd
:KK{c1}KKKKKKKKKK0o;...^cdxxOK0O/^^'  {c2}.0K:
 kKK{c1}KKKKKKKKKKKKK0x;,,......,;od  {c2}lKk
 '0K{c1}KKKKKKKKKKKKKKKKKKKK00KKOo^  {c2}c00'
  'kK{c1}KKOxddxkOO00000Okxoc;''   {c2}.dKk'
    l0Ko.                    .c00l'
     'l0Kk:.              .;xK0l'
        'lkK0xl:;,,,,;:ldO0kl'
            '^:ldxkkkkxdl:^'"##########
    )
}

pub fn swag_arch() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c2}        .;ldkOKXXNNNNXXK0Oxoc,.
   ,lkXMMNK0OkkxkkOKWMMMMMMMMMM;
 'K0xo  ..,;:c:.     `'lKMMMMM0
     .lONMMMMMM'         `lNMk'
{c2}    ;WMMMMMMMMMO.              {c1}....::...
{c2}    OMMMMMMMMMMMMKl.       {c1}.,;;;;;ccccccc,
{c2}    `0MMMMMMMMMMMMMM0:         {c1}.. .ccccccc.
{c2}      'kWMMMMMMMMMMMMMNo.   {c1}.,:'  .ccccccc.
{c2}        `c0MMMMMMMMMMMMMN,{c1},:c;    :cccccc:
{c2} ckl.      `lXMMMMMMMMMX{c1}occcc:.. ;ccccccc.
{c2}dMMMMXd,     `OMMMMMMWk{c1}ccc;:''` ,ccccccc:
{c2}XMMMMMMMWKkxxOWMMMMMNo{c1}ccc;     .cccccccc.
{c2} `':ldxO0KXXXXXK0Okdo{c1}cccc.     :cccccccc.
                    :ccc:'     `cccccccc:,
                                   ''"##########
    )
}

pub fn tails() -> String {
    let c1 = COLORS[5];
    format!(
        r##########"{c1}      ``
  ./yhNh
syy/Nshh         `:o/
N:dsNshh  █   `ohNMMd
N-/+Nshh      `yMMMMd
N-yhMshh       yMMMMd
N-s:hshh  █    yMMMMd so//.
N-oyNsyh       yMMMMd d  Mms.
N:hohhhd:.     yMMMMd  syMMM+
Nsyh+-..+y+-   yMMMMd   :mMM+
+hy-      -ss/`yMMMM     `+d+
  :sy/.     ./yNMMMMm      ``
    .+ys- `:+hNMMMMMMy/`
      `hNmmMMMMMMMMMMMMdo.
       dMMMMMMMMMMMMMMMMMNh:
       +hMMMMMMMMMMMMMMMMMmy.
         -oNMMMMMMMMMMmy+.`
           `:yNMMMds/.`
              .//`"##########
    )
}

pub fn trisquel() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[6];
    format!(
        r##########"{c1}                         ▄▄▄▄▄▄
                      ▄█████████▄
      ▄▄▄▄▄▄         ████▀   ▀████
   ▄██████████▄     ████▀   ▄▄ ▀███
 ▄███▀▀   ▀▀████     ███▄   ▄█   ███
▄███   ▄▄▄   ████▄    ▀██████   ▄███
███   █▀▀██▄  █████▄     ▀▀   ▄████
▀███      ███  ███████▄▄  ▄▄██████
{c1} ▀███▄   ▄███  █████████████{c2}████▀
{c1}  ▀█████████    ███████{c2}███▀▀▀
    ▀▀███▀▀     ██████▀▀
               ██████▀   ▄▄▄▄
              █████▀   ████████
              █████   ███▀  ▀███
               ████▄   ██▄▄▄  ███
                █████▄   ▀▀  ▄██
                  ██████▄▄▄████
                     ▀▀█████▀▀"##########
    )
}

pub fn ubuntu_cinnamon() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}            .-/+oooooooo+/-.
        `:+oooooooooooooooooo+:`
      -+oooooooooooooooooooooooo+-
    .ooooooooooooooooooo{c2}:ohNd{c1}oooooo.
   /oooooooooooo{c2}:/+oo++:/ohNd{c1}ooooooo/
  +oooooooooo{c2}:osNdhyyhdNNh+:+{c1}oooooooo+
 /ooooooooo{c2}/dN/{c1}ooooooooo{c2}/sNNo{c1}ooooooooo/
.ooooooooo{c2}oMd:{c1}oooooooooooo{c2}:yMy{c1}ooooooooo.
+ooooo{c2}:+o/Md{c1}oooooo{c2}:sm/{c1}oo/ooo{c2}yMo{c1}oooooooo+
ooo{c2}:sdMdosMo{c1}ooooo{c2}oNMd{c1}//{c2}dMd+{c1}o{c2}:so{c1}ooooooooo
oooo{c2}+ymdosMo{c1}ooo{c2}+mMm{c1}+/{c2}hMMMMMh+hs{c1}ooooooooo
+oooooo{c2}:{c1}:{c2}/Nm:{c1}/{c2}hMNo{c1}:y{c2}MMMMMMMMMM+{c1}oooooooo+
.ooooooooo{c2}/NNMNy{c1}:o{c2}NMMMMMMMMMMo{c1}ooooooooo.
/oooooooooo{c2}:yh:{c1}+m{c2}MMMMMMMMMMd/{c1}ooooooooo/
  +oooooooooo{c2}+{c1}/h{c2}mMMMMMMNds//o{c1}oooooooo+
   /oooooooooooo{c2}+:////:o/ymMd{c1}ooooooo/
    .oooooooooooooooooooo{c2}/sdh{c1}oooooo.
      -+oooooooooooooooooooooooo+-
        `:+oooooooooooooooooo+:`
            .-/+oooooooo+/-."##########
    )
}

pub fn ubuntu_budgie() -> String {
    let c2 = COLORS[7];
    format!(
        r##########"{c2}           ./oydmMMMMMMmdyo/.
        :smMMMMMMMMMMMhs+:++yhs:
     `omMMMMMMMMMMMN+`        `odo`
    /NMMMMMMMMMMMMN-            `sN/
  `hMMMMmhhmMMMMMMh               sMh`
 .mMmo-     /yMMMMm`              `MMm.
 mN/       yMMMMMMMd-              MMMm
oN-        oMMMMMMMMMms+//+o+:    :MMMMo
m/          +NMMMMMMMMMMMMMMMMm. :NMMMMm
M`           .NMMMMMMMMMMMMMMMNodMMMMMMM
M-            sMMMMMMMMMMMMMMMMMMMMMMMMM
mm`           mMMMMMMMMMNdhhdNMMMMMMMMMm
oMm/        .dMMMMMMMMh:      :dMMMMMMMo
 mMMNyo/:/sdMMMMMMMMM+          sMMMMMm
 .mMMMMMMMMMMMMMMMMMs           `NMMMm.
  `hMMMMMMMMMMM.oo+.            `MMMh`
    /NMMMMMMMMMo                sMN/
     `omMMMMMMMMy.            :dmo`
        :smMMMMMMMh+-`   `.:ohs:
           ./oydmMMMMMMdhyo/."##########
    )
}

pub fn ubuntu_gnome() -> String {
    let c3 = COLORS[7];
    let c4 = COLORS[6];
    format!(
        r##########"{c3}          ./o.
        .oooooooo
      .oooo```soooo
    .oooo`     `soooo
   .ooo`   {c4}.o.{c3}   `\/ooo.
   :ooo   {c4}:oooo.{c3}   `\/ooo.
    sooo    {c4}`ooooo{c3}    \/oooo
     \/ooo    {c4}`soooo{c3}    `ooooo
      `soooo    {c4}`\/ooo{c3}    `soooo
{c4}./oo    {c3}`\/ooo    {c4}`/oooo.{c3}   `/ooo
{c4}`\/ooo.   {c3}`/oooo.   {c4}`/oooo.{c3}   ``
{c4}  `\/ooo.    {c3}/oooo     {c4}/ooo`
{c4}     `ooooo    {c3}``    {c4}.oooo
{c4}       `soooo.     .oooo`
         `\/oooooooooo`
            ``\/oo``"##########
    )
}

pub fn ubuntu_mate() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}            .:/+oossssoo+/:.`
        `:+ssssssssssssssssss+:`
      -+sssssssssssssss{c2}y{c1}ssssssss+-
    .osssssssssssss{c2}yy{c1}ss{c2}mMmh{c1}ssssssso.
   /sssssssss{c2}ydmNNNmmd{c1}s{c2}mMMMMNdy{c1}sssss/
 `+ssssssss{c2}hNNdy{c1}sssssss{c2}mMMMMNdy{c1}ssssss+`
 +sssssss{c2}yNNh{c1}ss{c2}hmNNNNm{c1}s{c2}mMmh{c1}s{c2}ydy{c1}sssssss+
-sssss{c2}y{c1}ss{c2}Nm{c1}ss{c2}hNNh{c1}ssssss{c2}y{c1}s{c2}hh{c1}ss{c2}mMy{c1}sssssss-
+ssss{c2}yMNdy{c1}ss{c2}hMd{c1}ssssssssss{c2}hMd{c1}ss{c2}NN{c1}sssssss+
sssss{c2}yMMMMMmh{c1}sssssssssssss{c2}NM{c1}ss{c2}dMy{c1}sssssss
sssss{c2}yMMMMMmhy{c1}ssssssssssss{c2}NM{c1}ss{c2}dMy{c1}sssssss
+ssss{c2}yMNdy{c1}ss{c2}hMd{c1}ssssssssss{c2}hMd{c1}ss{c2}NN{c1}sssssss+
-sssss{c2}y{c1}ss{c2}Nm{c1}ss{c2}hNNh{c1}ssssssss{c2}dh{c1}ss{c2}mMy{c1}sssssss-
 +sssssss{c2}yNNh{c1}ss{c2}hmNNNNm{c1}s{c2}mNmh{c1}s{c2}ymy{c1}sssssss+
  +ssssssss{c2}hNNdy{c1}sssssss{c2}mMMMMmhy{c1}ssssss+
   /sssssssss{c2}ydmNNNNmd{c1}s{c2}mMMMMNdh{c1}sssss/
    .osssssssssssss{c2}yy{c1}ss{c2}mMmdy{c1}sssssso.
      -+sssssssssssssss{c2}y{c1}ssssssss+-
        `:+ssssssssssssssssss+:`
            .:/+oossssoo+/:."##########
    )
}

pub fn ubuntu_old() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    let c3 = COLORS[3];
    format!(
        r##########"{c1}                         ./+o+-
{c2}                 yyyyy- {c1}-yyyyyy+
{c2}              {c2}://+//////{c1}-yyyyyyo
{c3}          .++ {c2}.:/++++++/-{c1}.+sss/`
{c3}        .:++o:  {c2}/++++++++/:--:/-
{c3}       o:+o+:++.{c2}`..```.-/oo+++++/
{c3}      .:+o:+o/.{c2}          `+sssoo+/
{c2} .++/+:{c3}+oo+o:`{c2}             /sssooo.
{c2}/+++//+:{c3}`oo+o{c2}               /::--:.
{c2}+/+o+++{c3}`o++o{c1}               ++////.
{c2} .++.o+{c3}++oo+:`{c1}             /dddhhh.
{c3}      .+.o+oo:.{c1}          `oddhhhh+
{c3}       +.++o+o`{c1}`-````.:ohdhhhhh+
{c3}        `:o+++ {c1}`ohhhhhhhhyo++os:
{c3}          .o:{c1}`.syhhhhhhh/{c3}.oo++o`
{c1}              /osyyyyyyo{c3}++ooo+++/
{c1}                  ````` {c3}+oo+++o:
{c3}                         `oo++."##########
    )
}

pub fn ubuntu_studio() -> String {
    let c1 = COLORS[6];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}              ..-::::::-.`
         `.:+++++++++++{c2}ooo{c1}++:.`
       ./+++++++++++++{c2}sMMMNdyo{c1}+/.
     .++++++++++++++++{c2}oyhmMMMMms{c1}++.
   `/+++++++++{c2}osyhddddhys{c1}+{c2}osdMMMh{c1}++/`
  `+++++++++{c2}ydMMMMNNNMMMMNds{c1}+{c2}oyyo{c1}++++`
  +++++++++{c2}dMMNhso{c1}++++{c2}oydNMMmo{c1}++++++++`
 :+{c2}odmy{c1}+++{c2}ooysoohmNMMNmyoohMMNs{c1}+++++++:
 ++{c2}dMMm{c1}+{c2}oNMd{c1}++{c2}yMMMmhhmMMNs+yMMNo{c1}+++++++
`++{c2}NMMy{c1}+{c2}hMMd{c1}+{c2}oMMMs{c1}++++{c2}sMMN{c1}++{c2}NMMs{c1}+++++++.
`++{c2}NMMy{c1}+{c2}hMMd{c1}+{c2}oMMMo{c1}++++{c2}sMMN{c1}++{c2}mMMs{c1}+++++++.
 ++{c2}dMMd{c1}+{c2}oNMm{c1}++{c2}yMMNdhhdMMMs{c1}+y{c2}MMNo{c1}+++++++
 :+{c2}odmy{c1}++{c2}oo{c1}+{c2}ss{c1}+{c2}ohNMMMMmho{c1}+{c2}yMMMs{c1}+++++++:
  +++++++++{c2}hMMmhs+ooo+oshNMMms{c1}++++++++
  `++++++++{c2}oymMMMMNmmNMMMMmy+oys{c1}+++++`
   `/+++++++++{c2}oyhdmmmmdhso+sdMMMs{c1}++/
     ./+++++++++++++++{c2}oyhdNMMMms{c1}++.
       ./+++++++++++++{c2}hMMMNdyo{c1}+/.
         `.:+++++++++++{c2}sso{c1}++:.
              ..-::::::-.."##########
    )
}

pub fn ubuntu_small() -> String {
    let c1 = COLORS[1];
    format!(
        r##########"{c1}         _
     ---(_)
 _/  ---  \\
(_) |   |
  \\  --- _/
     ---(_)"##########
    )
}

pub fn ubuntu() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}            .-/+oossssoo+\-.
        ´:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssss{c2}dMMMNy{c1}sssso.
   /sssssssssss{c2}hdmmNNmmyNMMMMh{c1}ssssss\
  +sssssssss{c2}hm{c1}yd{c2}MMMMMMMNddddy{c1}ssssssss+
 /ssssssss{c2}hNMMM{c1}yh{c2}hyyyyhmNMMMNh{c1}ssssssss\
.ssssssss{c2}dMMMNh{c1}ssssssssss{c2}hNMMMd{c1}ssssssss.
+ssss{c2}hhhyNMMNy{c1}ssssssssssss{c2}yNMMMy{c1}sssssss+
oss{c2}yNMMMNyMMh{c1}ssssssssssssss{c2}hmmmh{c1}ssssssso
oss{c2}yNMMMNyMMh{c1}sssssssssssssshmmmh{c1}ssssssso
+ssss{c2}hhhyNMMNy{c1}ssssssssssss{c2}yNMMMy{c1}sssssss+
.ssssssss{c2}dMMMNh{c1}ssssssssss{c2}hNMMMd{c1}ssssssss.
 \ssssssss{c2}hNMMM{c1}yh{c2}hyyyyhdNMMMNh{c1}ssssssss/
  +sssssssss{c2}dm{c1}yd{c2}MMMMMMMMddddy{c1}ssssssss+
   \sssssssssss{c2}hdmNNNNmyNMMMMh{c1}ssssss/
    .ossssssssssssssssss{c2}dMMMNy{c1}sssso.
      -+sssssssssssssssss{c2}yyy{c1}ssss+-
        `:+ssssssssssssssssss+:`
            .-\+oossssoo+/-."##########
    )
}

pub fn univention() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}         ./osssssssssssssssssssssso+-
       `ohhhhhhhhhhhhhhhhhhhhhhhhhhhhy:
       shhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh-
   `-//{c2}sssss{c1}/hhhhhhhhhhhhhh+{c2}s{c1}.hhhhhhhhh+
 .ohhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sss{c1}+hhhhhhh+
.yhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}ssss{c1}:hhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}sssss{c1}.hhhhhhhhhhhhhh.{c2}sssss{c1}yhhhhh+
+hhhhhy{c2}ssssss{c1}+yhhhhhhhhhhy/{c2}ssssss{c1}yhhhhh+
+hhhhhh:{c2}sssssss{c1}:hhhhhhh+{c2}.ssssssss{c1}yhhhhy.
+hhhhhhh+`{c2}ssssssssssssssss{c1}hh{c2}sssss{c1}yhhho`
+hhhhhhhhhs+{c2}ssssssssssss{c1}+hh+{c2}sssss{c1}/:-`
-hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhho
 :yhhhhhhhhhhhhhhhhhhhhhhhhhhhh+`
   -+ossssssssssssssssssssss+:`"##########
    )
}

pub fn venom() -> String {
    let c1 = COLORS[8];
    format!(
        r##########"{c1}   :::::::          :::::::
   mMMMMMMm        dMMMMMMm
   /MMMMMMMo      +MMMMMMM/
    yMMMMMMN      mMMMMMMy
     NMMMMMMs    oMMMMMMm
     +MMMMMMN:   NMMMMMM+
      hMMMMMMy  sMMMMMMy
      :NMMMMMM::NMMMMMN:
       oMMMMMMyyMMMMMM+
        dMMMMMMMMMMMMh
        /MMMMMMMMMMMN:
         sMMMMMMMMMMo
          mMMMMMMMMd
          +MMMMMMMN:
            ::::::"##########
    )
}

pub fn void_small() -> String {
    let c1 = COLORS[2];
    format!(
        r##########"{c1}    _______
 _ \\______ -
| \\  ___  \\ |
| | /   \ | |
| | \___/ | |
| \\______ \\_|
 -_______\\"##########
    )
}

// pub fn Void() -> String {
// let c1 = Colors[2];
// let c2 = Colors[8];
//   format!(
//     r##########"{c1}                __.;=====;.__
//             _.=+==++=++=+=+===;.
//              -=+++=+===+=+=+++++=_
//         .     -=:``     `--==+=++==.
//        _vi,    `            --+=++++:
//       .uvnvi.       _._       -==+==+.
//      .vvnvnI`    .;==|==;.     :|=||=|.
// {c2}+QmQQm{c1}pvvnv; {c2}_yYsyQQWUUQQQm #QmQ#{c1}:{c2}QQQWUV$QQm.
// {c2} -QQWQW{c1}pvvo{c2}wZ?.wQQQE{c1}==<{c2}QWWQ/QWQW.QQWW{c1}(: {c2}jQWQE
// {c2}  -$QQQQmmU'  jQQQ@{c1}+=<{c2}QWQQ)mQQQ.mQQQC{c1}+;{c2}jWQQ@'
// {c2}   -$WQ8Y{c1}nI:   {c2}QWQQwgQQWV{c1}`{c2}mWQQ.jQWQQgyyWW@!
// {c1}     -1vvnvv.     `~+++`        ++|+++
//       +vnvnnv,                 `-|===
//        +vnvnvns.           .      :=-
//         -Invnvvnsi..___..=sv=.     `
//           +Invnvnvnnnnnnnnvvnn;.
//             ~|Invnvnvvnvvvnnv}+`
//                -~|{*l}*|~"##########
//   )
// }

// pub fn VNux() -> String {
// let c1 = Colors[11];
// let c2 = Colors[8];
// let c3 = Colors[15];
// let c4 = Colors[1];
// let c5 = Colors[7];
//   format!(
//     r##########"{c1}              `
//            ^[XOx~.
//         ^_nwdbbkp0ti'
//         <vJCZw0LQ0Uj>
// {c2}          _j>!vC1,,
//      {c4},{c2}   ,CY{c3}O{c2}t{c3}O{c2}1(l;"
// `{c4}~-{r(1I{c2} ^{c1}/zmwJuc:{c2}I^
// '{c4}?)|{c1}U{c4}/}-{c2} ^{c3}f{c1}OCLLOw{c3}_{c2},;
//  ,{c4}i,``. {c2}",{c3}k%ooW@$d"{c2}I,'
//   '    ;^{c3}u$$$$$$$$^<{c2}:^
//    ` .>>{c3}($${c5}$@@@@$${c3}$nl{c2}[::
//     `!}?{c3}B${c5}%&WMMW&%${c3}$1}-{c2}}":
//     ^?j{c3}Z${c5}WMMWWWWMMW${c3}ofc{c2};;`
//     <~x&{c3}${c5}&MWWWWWWWWp{c3}-{c5}l>[<
// {c1} 'ljmwn{c2}~tk8{c5}MWWWWM8O{c2}X{c1}r{c2}+]nC{c1}[
// !JZqwwdX{c2}:^C8{c5}#MMMM@{c2}X{c1}Odpdpq0<
// <wwwwmmpO{c2}1{c3}0@%%%%8{c2}d{c1}nqmwmqqqJl
// ?QOZmqqqpb{c2}t[run/?!{c1}0pwqqQj-,
//  ^:l<{nUUv>      ^x00J("
//                    ^""##########
//   )
// }

pub fn langit_ketujuh() -> String {
    let c1 = COLORS[7];
    let c2 = COLORS[4];
    format!(
        r##########"{c1}
   L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L
      'L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L
   L7L.   'L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L
   L7L7L7L                             L7L7L7L
   L7L7L7L                             L7L7L7L
   L7L7L7L             L7L7L7L7L7L7L7L7L7L7L7L
   L7L7L7L                'L7L7L7L7L7L7L7L7L7L
   L7L7L7L                    'L7L7L7L7L7L7L7L
   L7L7L7L                             L7L7L7L
   L7L7L7L                             L7L7L7L
   L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L.   'L7L
   L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L.
   L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L7L
{c2}"##########
    )
}

pub fn semc() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[8];
    let c3 = COLORS[1];
    format!(
        r##########"{c1}            /\
     ______/  \
    /      |()| {c2}E M C
{c1}   |   (-- |  |
    \   \  |  |
.----)   | |__|
|_______/ / {c3}"{c1}  \
              {c3}"
            ""##########
    )
}

pub fn obarun() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                    ,;::::;
                ;cooolc;,
             ,coool;
           ,loool,
          loooo;
        :ooool
       cooooc            ,:ccc;
      looooc           :oooooool
     cooooo          ;oooooooooo,
    :ooooo;         :ooooooooooo
    oooooo          oooooooooooc
   :oooooo         :ooooooooool
   loooooo         ;oooooooool
   looooooc        .coooooooc
   cooooooo:           ,;co;
   ,ooooooool;       ,:loc
    cooooooooooooloooooc
     ;ooooooooooooool;
       ;looooooolc;"##########
    )
}

pub fn windows11() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################

################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################"##########
    )
}

pub fn windows10() -> String {
    let c1 = COLORS[6];
    format!(
        r##########"{c1}                                ..,
                    ....,,:;+ccllll
      ...,,+:;  cllllllllllllllllll
,cclllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll

llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
`'ccllllllllll  lllllllllllllllllll
       `' \\*::  :ccllllllllllllllll
                       ````''*::cll
                                 ``"##########
    )
}

pub fn windows() -> String {
    let c1 = COLORS[1];
    let c2 = COLORS[2];
    let c3 = COLORS[4];
    let c4 = COLORS[3];
    format!(
        r##########"{c1}        ,.=:!!t3Z3z.,
       :tt:::tt333EE3
{c1}       Et:::ztt33EEEL${c2} @Ee.,      ..,
{c1}      ;tt:::tt333EE7{c2} ;EEEEEEttttt33#
{c1}     :Et:::zt333EEQ.{c2} $EEEEEttttt33QL
{c1}     it::::tt333EEF{c2} @EEEEEEttttt33F
{c1}    ;3=*^```"*4EEV{c2} :EEEEEEttttt33@.
{c3}    ,.=::::!t=., {c1}`{c2} @EEEEEEtttz33QF
{c3}   ;::::::::zt33){c2}   "4EEEtttji3P*
{c3}  :t::::::::tt33.{c4}:Z3z..{c2}  ``{c4} ,..g.
{c3}  i::::::::zt33F{c4} AEEEtttt::::ztF
{c3} ;:::::::::t33V{c4} ;EEEttttt::::t3
{c3} E::::::::zt33L{c4} @EEEtttt::::z3F
{c3}{{3=*^```"*4E3){c4} ;EEEtttt:::::tZ`
{c3}             `{c4} :EEEEtttt::::z7
                 "VEzjt:;;z>*`"##########
    )
}

pub fn xubuntu() -> String {
    let c1 = COLORS[4];
    let c2 = COLORS[7];
    format!(
        r##########"{c1}           `.:/ossyyyysso/:.
        `.yyyyyyyyyyyyyyyyyyyy.`
      `yyyyyyyyyyyyyyyyyyyyyyyyyy`
    `yyyyyyyyyyyyyyyyyyyy{c2}::{c1}yyyyyyyy`
   .yyyyyyyyyyy{c2}/+:{c1}yyyyyyy{c2}ds{c1}yyy{c2}+y{c1}yyyy.
  yyyyyyy{c2}:o/{c1}yy{c2}dMMM+{c1}yyyyy{c2}/M+{c1}y{c2}:hM+{c1}yyyyyy
 yyyyyyy{c2}+MMMy{c1}y{c2}mMMMh{c1}yyyyy{c2}yM::mM+{c1}yyyyyyyy
`yyyyyyy{c2}+MMMMysMMMd{c1}yyyyy{c2}dh:mN+{c1}yyyyyyyyy`
yyyyyyyy{c2}:NMMMMmMMMMmmdhyy+/y:{c1}yyyyyyyyyyy
yyyyyyyy{c2}+MMMMMMMMMMMMMMMMMMNho:{c1}yyyyyyyyy
yyyyyyyy{c2}mMMMMMMMMMMMMMMMMMMMMMMy{c1}yyyyyyyy
yyyyyyy{c2}+MMMMMMMMMMMMMMMMMMMMMMMM/{c1}yyyyyyy
`yyyyyy{c2}sMMMMMMMMMMMMMMMMMMMMMMmo{c1}yyyyyyy`
 yyyyyy{c2}oMMMMMMMMMMMMMMMMMMMmy+{c1}yyyyyyyyy
  yyyyy{c2}:mMMMMMMMMMMMMMMNho/{c1}yyyyyyyyyyy
   .yyyy{c2}:yNMMMMMMMNdyo:{c1}yyyyyyyyyyyyy.
    `yyyyyy{c2}:/++/::{c1}yyyyyyyyyyyyyyyyy`
      `yyyyyyyyyyyyyyyyyyyyyyyyyy`
        `.yyyyyyyyyyyyyyyyyyyy.`
           `.:/oosyyyysso/:.`"##########
    )
}

pub fn zorin() -> String {
    let c1 = COLORS[4];
    format!(
        r##########"{c1}        `osssssssssssssssssssso`
       .osssssssssssssssssssssso.
      .+oooooooooooooooooooooooo+.


  `::::::::::::::::::::::.         .:`
 `+ssssssssssssssssss+:.`     `.:+ssso`
.ossssssssssssssso/.       `-+ossssssso.
ssssssssssssso/-`      `-/osssssssssssss
.ossssssso/-`      .-/ossssssssssssssso.
 `+sss+:.      `.:+ssssssssssssssssss+`
  `:.         .::::::::::::::::::::::`


      .+oooooooooooooooooooooooo+.
       -osssssssssssssssssssssso-
        `osssssssssssssssssssso`"##########
    )
}

pub fn darwin() -> String {
    let c1 = COLORS[2];
    let c2 = COLORS[3];
    let c3 = COLORS[1];
    let c4 = COLORS[1];
    let c5 = COLORS[5];
    let c6 = COLORS[4];
    format!(
        r##########"{c1}                    c.'
                 ,xNMM.
               .OMMMMo
               lMMM"
     .;loddo:.  .olloddol;.
   cKMMMMMMMMMMNWMMMMMMMMMM0:
{c2} .KMMMMMMMMMMMMMMMMMMMMMMMWd.
 XMMMMMMMMMMMMMMMMMMMMMMMX.
{c3};MMMMMMMMMMMMMMMMMMMMMMMM:
:MMMMMMMMMMMMMMMMMMMMMMMM:
{c4}.MMMMMMMMMMMMMMMMMMMMMMMMX.
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.
 {c5}'XMMMMMMMMMMMMMMMMMMMMMMMMMMk
  'XMMMMMMMMMMMMMMMMMMMMMMMMK.
    {c6}kMMMMMMMMMMMMMMMMMMMMMMd
     ;KMMMMMMMWXXWMMMMMMMk.
       "cooc*"    "*coo'""##########
    )
}

pub fn profelis_samba_box() -> String {
    let c1 = COLORS[3];
    format!(
        r##########"{c1}
                    #
               *////#####
           /////////#########(
      .((((((/////    ,####(#(((((
  /#######(((*             (#(((((((((.
//((#(#(#,        ((##(        ,((((((//
//////        #(##########(       //////
//////    ((#(#(#(#(##########(/////////
/////(    (((((((#########(##((((((/////
/(((#(                             ((((/
####(#                             ((###
#########(((/////////(((((((((,    (#(#(
########(   /////////(((((((*      #####
####///,        *////(((         (((((((
.///////////                .//(((((((((
     ///////////,       *(/////((((*
         ,/(((((((((##########/.
             .((((((#######
                  ((##*"##########
    )
}
