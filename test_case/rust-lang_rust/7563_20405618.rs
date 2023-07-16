
blorg.rs:15:40: 15:64 error: internal compiler error: Cannot relate bound region as subregion: br_self
blorg.rs:15     fn get_pa(&self) -> &'self IDummy { self.pa as &'self IDummy }
                                                    ^~~~~~~~~~~~~~~~~~~~~~~~
