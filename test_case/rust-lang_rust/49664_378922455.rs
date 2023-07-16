text
20:51 < Gankro> acrichto: why is the is_x86_feature_detected! so precisely named? Why can't it be is_feature_detected?
20:55 <~acrichto> Gankro: it doesn't work on other paltforms
20:55 <~acrichto> in that the arguments are x86-specific
20:55 <~acrichto> (so there's a macro for each arch)
20:56 < Gankro> acrichto: why couldn't it just return false on the plats?
20:57 < Gankro> *other plats
20:57 <~acrichto> Gankro: catches typos and such
20:57 <~acrichto> in that it's currently a deliberate decision
20:57 <~acrichto> (we've discussed this a bunch historically)
20:57 < Gankro> ok
20:57 <~acrichto> it turns out that if the macros is cross paltform it doesn't really help at all
20:57 <~acrichto> it's only invoked in platform-specific contexts anyway
