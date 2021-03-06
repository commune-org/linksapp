use sycamore::prelude::*;

#[component(FooterWidget<G>)]
pub fn FooterWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
    footer (class="footer") {div (class="py-5") {
                    div (class="container") {
                        div (class="row align-items-center") {
                            div (class="col-md-7") {
                                div (class="section-title") {
                                    div (class="d-flex") {
                                        i (class="uil uil-mobile-android display-4 text-white title-dark") {}

                                        div (class="flex-1 ms-md-4 ms-3") {
                                            h4 (class="fw-bold text-white mb-1") {"Download our app now !"}

                                            p (class="text-white-50 mb-0") {"Build professional looking micro-site fast and easy using LinksApp."}

                                        }

                                    }

                                }

                            }


                            div (class="col-md-5 mt-4 mt-sm-0") {
                                div (class="text-md-end ms-5 ms-sm-0") {
                                    a (href="javascript:void(0)", class="btn btn-primary me-2 me-lg-2 me-md-0 my-2") {i (class="uil uil-apple") {}
    " App Store"}

                                    a (href="javascript:void(0)", class="btn btn-soft-primary my-2") {i (class="uil uil-google-play") {}
    " Play Store"}

                                }

                            }

                        }

                    }

                }


                div (class="container") {
                    div (class="row") {
                        div (class="col-12") {
                            div (class="footer-py-60 footer-border") {
                                div (class="row") {
                                    div (class="col-lg-4 col-12 mb-0 mb-md-4 pb-0 pb-md-2") {
                                        a (href="#", class="logo-footer") {
                                            img (src="assets/images/logo-light.png", height="24", alt="") {}
    }

                                        p (class="mt-4") {"Start working with LinksApp that can provide everything you need to generate awareness, drive traffic, connect."}

                                        ul (class="list-unstyled social-icon foot-social-icon mb-0 mt-4") {li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded") {i (data-feather="facebook", class="fea icon-sm fea-social") {}
    }
    }

                                            li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded") {i (data-feather="instagram", class="fea icon-sm fea-social") {}
    }
    }

                                            li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded") {i (data-feather="twitter", class="fea icon-sm fea-social") {}
    }
    }

                                            li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded") {i (data-feather="linkedin", class="fea icon-sm fea-social") {}
    }
    }

                                        }
    }


                                    div (class="col-lg-2 col-md-4 col-12 mt-4 mt-sm-0 pt-2 pt-sm-0") {
                                        h5 (class="footer-head") {"Company"}

                                        ul (class="list-unstyled footer-list mt-4") {li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
    " About us"}
    }

                                            li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
    " Services"}
    }

                                            li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
     "Team"}
    }

                                            li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
     "Pricing"}
    }


                                        }
    }


                                    div (class="col-lg-3 col-md-4 col-12 mt-4 mt-sm-0 pt-2 pt-sm-0") {
                                        h5 (class="footer-head") {"Usefull Links"}

                                        ul (class="list-unstyled footer-list mt-4") {li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
    " Terms of Services"}
    }

                                            li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
    " Privacy Policy"}
    }

                                            li  {a (href="javascript:void(0)", class="text-foot") {i (class="uil uil-angle-right-b me-1") {}
    " Documentation"}
    }


                                        }
    }


                                    div (class="col-lg-3 col-md-4 col-12 mt-4 mt-sm-0 pt-2 pt-sm-0") {
                                        h5 (class="footer-head") {"Newsletter"}

                                        p (class="mt-4") {"Sign up and receive the latest tips via email."}

                                        form  {
                                            div (class="row") {
                                                div (class="col-lg-12") {
                                                    div (class="foot-subscribe mb-3") {
                                                        label (class="form-label") {"Write your email" span (class="text-danger") {"*"}
    }

                                                        div (class="form-icon position-relative") {
                                                            i (data-feather="mail", class="fea icon-sm icons") {}

                                                            input (type="email", name="email", id="emailsubscribe", class="form-control ps-5 rounded", placeholder="Your email : ") {}
    }

                                                    }

                                                }

                                                div (class="col-lg-12") {
                                                    div (class="d-grid") {
                                                        input (type="submit", id="submitsubscribe", name="send", class="btn btn-soft-primary", value="Subscribe") {}
    }

                                                }

                                            }

                                        }

                                    }

                                }

                            }

                        }

                    }

                }


                div (class="footer-py-30 footer-bar") {
                    div (class="container text-center") {
                        div (class="row align-items-center") {
                            div (class="col-sm-6") {
                                div (class="text-sm-start") {
                                    p (class="mb-0") {"2022"
     "LinksApp" i (class="mdi mdi-heart text-danger") {}

    }

                                }

                            }


                            div (class="col-sm-6 mt-4 mt-sm-0 pt-2 pt-sm-0") {
                                ul (class="list-unstyled text-sm-end mb-0") {li (class="list-inline-item") {a (href="javascript:void(0)") {img (src="assets/images/payments/american-ex.png", class="avatar avatar-ex-sm", title="American Express", alt="") {}
    }
    }

                                    li (class="list-inline-item") {a (href="javascript:void(0)") {img (src="assets/images/payments/discover.png", class="avatar avatar-ex-sm", title="Discover", alt="") {}
    }
    }

                                    li (class="list-inline-item") {a (href="javascript:void(0)") {img (src="assets/images/payments/master-card.png", class="avatar avatar-ex-sm", title="Master Card", alt="") {}
    }
    }

                                    li (class="list-inline-item") {a (href="javascript:void(0)") {img (src="assets/images/payments/paypal.png", class="avatar avatar-ex-sm", title="Paypal", alt="") {}
    }
    }

                                    li (class="list-inline-item") {a (href="javascript:void(0)") {img (src="assets/images/payments/visa.png", class="avatar avatar-ex-sm", title="Visa", alt="") {}
    }
    }

                                }
    }

                        }

                    }

                }

            }



            }
}
