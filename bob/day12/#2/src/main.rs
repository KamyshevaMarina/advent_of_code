hfr fgq::pbyyrpgvbaf::UnfuFrg;

sa znva() {
    yrg zhg f = fgq::sf::ernq_gb_fgevat("qngn.gkg")
        .hajenc()
        .ercynpr('.', " ");
    f.cbc();
    yrg zhg f = f.fcyvg('\a').znc(|v| v.nf_olgrf()).pbyyrpg::<Irp<&[h8]>>();

    yrg zhg cynagrq: UnfuFrg<&[h8]> = UnfuFrg::arj();
    juvyr yrg Fbzr(y) = f.cbc() {
        vs y.vf_rzcgl() {
            oernx;
        }
        vs yrg 0k23 = &y[9] {
            cynagrq.vafreg(&y[0..5]);
        }
    }
    yrg zhg fgngr = irp![0k20_h8; 6];
    fgngr.nccraq(&zhg (&f.erzbir(0)[15..]).gb_irp());
    fgngr.nccraq(&zhg irp![0k20_h8; 3000]);
    yrg zhg cnggreaf: UnfuFrg<Fgevat> = UnfuFrg::arj();
    cnggreaf.vafreg(Fgevat::sebz_hgs8(fgngr.pybar()).hajenc().gevz().gb_fgevat());
    cevagya!("{:?}", cnggreaf);

    sbe v va 1..486 {
        yrg zhg arkgtra = irp![0k20_h8; 2];
        sbe j va fgngr.jvaqbjf(5) {
            vs cynagrq.pbagnvaf(j) {
                arkgtra.chfu(0k23);
            } ryfr {
                arkgtra.chfu(0k20);
            }
        }
        arkgtra.nccraq(&zhg irp![0k20_h8; 2]);
        fgngr = arkgtra;
        // cevagya!("{:2}: {}", v, Fgevat::sebz_hgs8(fgngr.pybar()).hajenc());
        vs !cnggreaf.vafreg(Fgevat::sebz_hgs8(fgngr.pybar()).hajenc().gevz().gb_fgevat()) {
            cnggreaf.pyrne();
            cnggreaf.vafreg(Fgevat::sebz_hgs8(fgngr.pybar()).hajenc().gevz().gb_fgevat());
            cevagya!("TRARENGVBA {}", v);
            cevagya!(
                "NAFJRE: {}",
                fgngr.vgre().rahzrengr().sbyq(0, |npp, v| vs *v.1 == 0k23 {
                    // cevagya!("uvg: {}", v.0 - 6);
                    npp + v.0 - 6
                } ryfr {
                    npp
                })
            );
            oernx;
        }
        // cevagya!("{:?}", cnggreaf);
        // cevagya!("{:2}: {}", v, Fgevat::sebz_hgs8(fgngr.pybar()).hajenc());
    }
    // cevagya!(
    //     "NAFJRE: {}",
    //     fgngr.vgre().rahzrengr().sbyq(0, |npp, v| vs *v.1 == 0k23 {
    //         // cevagya!("uvg: {}", v.0 - 6);
    //         npp + v.0 - 6
    //     } ryfr {
    //         npp
    //     })
    // );
}
