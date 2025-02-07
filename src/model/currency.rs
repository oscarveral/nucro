/// Macro to generate the Currency enum.
macro_rules! currency {
    ($($(#[$desc:meta])* ($code: ident, $num: literal, $decimal: literal)),+) => {
        /// All posible currencies with their numeric codes. ISO 4217.
        #[allow(clippy::upper_case_acronyms)]
        #[derive(PartialEq, Eq)]
        pub enum Currency {
            $(
                $(#[$desc])?
                $code = $num,
            )+
        }

        impl Currency {
            /// Obtain the currency code as a string literal.
            fn code(&self) -> &str  {
                match self {
                    $(
                        Currency::$code => stringify!($code)
                    ),*
                }
            }
            // Obtain the decimal places of the currency.
            fn decimal(&self) -> u8 {
                match self {
                    $(
                        Currency::$code => $decimal
                    ),*
                }
            }
        }
    };
}

currency!(
	/// United Arab Emirates dirham. United Arab Emirates.
	(AED, 784, 2),
	/// Afghan afghani. Afghanistan.
	(AFN, 971, 2),
	/// Albanian lek. Albania.
	(ALL, 8, 2),
	/// Armenian dram. Armenia.
	(AMD, 51, 2),
	/// Netherlands Antillean guilder. Curaçao, Sint Maarten.
	(ANG, 532, 2),
	/// Angolan kwanza. Angola.
	(AOA, 973, 2),
	/// Argentine peso. Argentina.
	(ARS, 32, 2),
	/// Australian dollar. Australia, Christmas Island, Cocos (Keeling) Islands, Heard Island and McDonald Islands, Kiribati, Nauru, Norfolk Island, Tuvalu.
	(AUD, 36, 2),
	/// Aruban florin. Aruba.
	(AWG, 533, 2),
	/// Azerbaijani manat. Azerbaijan.
	(AZN, 944, 2),
	/// Bosnia and Herzegovina convertible mark. Bosnia and Herzegovina.
	(BAM, 977, 2),
	/// Barbados dollar. Barbados.
	(BBD, 52, 2),
	/// Bangladeshi taka. Bangladesh.
	(BDT, 50, 2),
	/// Bulgarian lev. Bulgaria.
	(BGN, 975, 2),
	/// Bahraini dinar. Bahrain.
	(BHD, 48, 3),
	/// Burundian franc. Burundi.
	(BIF, 108, 0),
	/// Bermudian dollar. Bermuda.
	(BMD, 60, 2),
	/// Brunei dollar. Brunei.
	(BND, 96, 2),
	/// Boliviano. Bolivia.
	(BOB, 68, 2),
	/// Bolivian Mvdol. Bolivia.
	(BOV, 984, 2),
	/// Brazilian real. Brazil.
	(BRL, 986, 2),
	/// Bahamian dollar. Bahamas.
	(BSD, 44, 2),
	/// Bhutanese ngultrum. Bhutan.
	(BTN, 64, 2),
	/// Botswana pula. Botswana.
	(BWP, 72, 2),
	/// Belarusian ruble. Belarus.
	(BYN, 933, 2),
	/// Belize dollar. Belize.
	(BZD, 84, 2),
	/// Canadian dollar. Canada.
	(CAD, 124, 2),
	/// Congolese franc. Democratic Republic of the Congo.
	(CDF, 976, 2),
	/// WIR euro. Switzerland.
	(CHE, 947, 2),
	/// Swiss franc. Switzerland, Liechtenstein.
	(CHF, 756, 2),
	/// WIR franc. Switzerland.
	(CHW, 848, 2),
	/// Unidad de Fomento. Chile.
	(CLF, 990, 4),
	/// Chilean peso. Chile.
	(CLP, 152, 0),
	/// Renminbi. China.
	(CNY, 156, 2),
	/// Colombian peso. Colombia.
	(COP, 170, 2),
	/// Unidad de Valor Real. Colombia.
	(COU, 970, 2),
	/// Costa Rican colon. Costa Rica.
	(CRC, 188, 2),
	/// Cuban peso. Cuba.
	(CUP, 192, 2),
	/// Cape Verdean escudo. Cape Verde.
	(CVE, 132, 2),
	/// Czech koruna. Czech Republic.
	(CZK, 203, 2),
	/// Djiboutian franc. Djibouti.
	(DJF, 262, 0),
	/// Danish krone. Denmark, Faroe Islands, Greenland.
	(DKK, 208, 2),
	/// Dominican peso. Dominican Republic.
	(DOP, 214, 2),
	/// Algerian dinar. Algeria.
	(DZD, 12, 2),
	/// Egyptian pound. Egypt.
	(EGP, 818, 2),
	/// Eritrean nakfa. Eritrea.
	(ERN, 232, 2),
	/// Ethiopian birr. Ethiopia.
	(ETB, 230, 2),
	/// Euro. Åland Islands, Andorra, Austria, Belgium, Croatia, Cyprus, Estonia, European Union, Finland, France, French Guiana, French Southern and Antarctic Lands, Germany, Greece, Guadeloupe, Ireland, Italy, Kosovo, Latvia, Lithuania, Luxembourg, Malta, Martinique, Mayotte, Monaco, Montenegro, Netherlands, Portugal, Réunion, Saint Barthélemy, Saint Martin, Saint Pierre and Miquelon, San Marino, Slovakia, Slovenia, Spain, Vatican City.
	(EUR, 978, 2),
	/// Fiji dollar. Fiji.
	(FJD, 242, 2),
	/// Falkland Islands pound. Falkland Islands.
	(FKP, 238, 2),
	/// Pound sterling. United Kingdom, Isle of Man, Jersey, Guernsey, Tristan da Cunha.
	(GBP, 826, 2),
	/// Georgian lari. Georgia.
	(GEL, 981, 2),
	/// Ghanaian cedi. Ghana.
	(GHS, 936, 2),
	/// Gibraltar pound. Gibraltar.
	(GIP, 292, 2),
	/// Gambian dalasi. Gambia.
	(GMD, 270, 2),
	/// Guinean franc. Guinea.
	(GNF, 324, 0),
	/// Guatemalan quetzal. Guatemala.
	(GTQ, 320, 2),
	/// Guyanese dollar. Guyana.
	(GYD, 328, 2),
	/// Hong Kong dollar. Hong Kong.
	(HKD, 344, 2),
	/// Honduran lempira. Honduras.
	(HNL, 340, 2),
	/// Haitian gourde. Haiti.
	(HTG, 332, 2),
	/// Hungarian forint. Hungary.
	(HUF, 348, 2),
	/// Indonesian rupiah. Indonesia.
	(IDR, 360, 2),
	/// Israeli new shekel. Israel.
	(ILS, 376, 2),
	/// Indian rupee. India, Bhutan.
	(INR, 356, 2),
	/// Iraqui dinar. Iraq.
	(IQD, 368, 3),
	/// Iranian rial. Iran.
	(IRR, 364, 2),
	/// Icelandic króna. Iceland.
	(ISK, 352, 0),
	/// Jamaican dollar. Jamaica.
	(JMD, 288, 2),
	/// Jordanian dinar. Jordan.
	(JOD, 400, 3),
	/// Japanese yen. Japan.
	(JPY, 392, 0),
	/// Kenyan shilling. Kenya.
	(KES, 404, 2),
	/// Kyrgyzstani som. Kyrgyzstan.
	(KGS, 417, 2),
	/// Cambodian riel. Cambodia.
	(KHR, 116, 2),
	/// Comoro franc. Comoros.
	(KMF, 174, 0),
	/// North Korean won. North Korea.
	(KPW, 408, 2),
	/// South Korean won. South Korea.
	(KRW, 410, 0),
	/// Kuwaiti dinar. Kuwait.
	(KWD, 414, 3),
	/// Cayman Islands dollar. Cayman Islands.
	(KYD, 136, 2),
	/// Kazakhstani tenge. Kazakhstan.
	(KZT, 398, 2),
	/// Lao kip. Laos.
	(LAK, 418, 2),
	/// Lebanese pound. Lebanon.
	(LBP, 422, 2),
	/// Sri Lankan rupee. Sri Lanka.
	(LKR, 144, 2),
	/// Liberian dollar. Liberia.
	(LRD, 430, 2),
	/// Lesotho loti. Lesotho.
	(LSL, 426, 2),
	/// Libyan dinar. Libya.
	(LYD, 434, 3),
	/// Moroccan dirham. Morocco, Western Sahara.
	(MAD, 504, 2),
	/// Moldovan leu. Moldova.
	(MDL, 498, 2),
	/// Malagasy ariary. Madagascar.
	(MGA, 969, 2),
	/// Macedonian denar. North Macedonia.
	(MKD, 807, 2),
	/// Myanmar kyat. Myanmar.
	(MMK, 104, 2),
	/// Mongolian tögrög. Mongolia.
	(MNT, 496, 2),
	/// Macanese pataca. Macau.
	(MOP, 446, 2),
	/// Mauritanian ouguiya. Mauritania.
	(MRU, 929, 2),
	/// Mauritian rupee. Mauritius.
	(MUR, 480, 2),
	/// Maldivian rufiyaa. Maldives.
	(MVR, 462, 2),
	/// Malawian kwacha. Malawi.
	(MWK, 454, 2),
	/// Mexican peso. Mexico.
	(MXN, 484, 2),
	/// Mexican Unidad de Inversión. Mexico.
	(MXV, 979, 2),
	/// Malaysian ringgit. Malaysia.
	(MYR, 458, 2),
	/// Mozambican metical. Mozambique.
	(MZN, 943, 2),
	/// Namibian dollar. Namibia.
	(NAD, 516, 2),
	/// Nigerian naira. Nigeria.
	(NGN, 566, 2),
	/// Nicaraguan córdoba. Nicaragua.
	(NIO, 558, 2),
	/// Norwegian krone. Norway, Svalbard and  Jan Mayen, Bouvet Island.
	(NOK, 578, 2),
	/// Nepalese rupee. Nepal.
	(NPR, 524, 2),
	/// New Zealand dollar. New Zealand, Cook Islands, Niue, Pitcairn Islands, Tokelau.
	(NZD, 554, 2),
	/// Omani rial. Oman.
	(OMR, 512, 3),
	/// Panamanian balboa. Panama.
	(PAB, 590, 2),
	/// Peruvian sol. Peru.
	(PEN, 604, 2),
	/// Papua New Guinean kina. Papua New Guinea.
	(PGK, 598, 2),
	/// Philippine peso. Philippines.
	(PHP, 608, 2),
	/// Pakistani rupee. Pakistan.
	(PKR, 586, 2),
	/// Polish złoty. Poland.
	(PLN, 985, 2),
	/// Paraguayan guaraní. Paraguay.
	(PYG, 600, 0),
	/// Qatari riyal. Qatar.
	(QAR, 634, 2),
	/// Romanian leu. Romania.
	(RON, 946, 2),
	/// Serbian dinar. Serbia.
	(RSD, 941, 2),
	/// Russian ruble. Russia.
	(RUB, 643, 2),
	/// Rwandan franc. Rwanda.
	(RWF, 646, 0),
	/// Saudi riyal. Saudi Arabia.
	(SAR, 682, 2),
	/// Solomon Islands dollar. Solomon Islands.
	(SBD, 90, 2),
	/// Seychelles rupee. Seychelles.
	(SCR, 690, 2),
	/// Sudanese pound. Sudan.
	(SDG, 938, 2),
	/// Swedish krona. Sweden.
	(SEK, 752, 2),
	/// Singapore dollar. Singapore.
	(SGD, 702, 2),
	/// Saint Helena pound. Saint Helena, Ascension Island.
	(SHP, 654, 2),
	/// Sierra Leonean leone. Sierra Leone.
	(SLE, 925, 2),
	/// Sierra Leonean leone. Somalia.
	(SOS, 706, 2),
	/// Surinamese dollar. Suriname.
	(SRD, 968, 2),
	/// South Sudanese pound. South Sudan.
	(SSP, 728, 2),
	/// São Tomé and Príncipe dobra. São Tomé and Príncipe.
	(STN, 930, 2),
	/// Salvadoran colón. El Salvador.
	(SVC, 222, 2),
	/// Syrian pound. Syria.
	(SYP, 760, 2),
	/// Swazi lilangeni. Eswatini.
	(SZL, 748, 2),
	/// Thai baht. Thailand.
	(THB, 764, 2),
	/// Tajikistani somoni. Tajikistan.
	(TJS, 972, 2),
	/// Turkmenistan manat. Turkmenistan.
	(TMT, 934, 2),
	/// Tunisian dinar. Tunisia.
	(TND, 788, 3),
	/// Tongan paʻanga. Tonga.
	(TOP, 776, 2),
	/// Turkish lira. Turkey.
	(TRY, 949, 2),
	/// Trinidad and Tobago dollar. Trinidad and Tobago.
	(TTD, 780, 2),
	/// New Taiwan dollar. Taiwan.
	(TWD, 901, 2),
	/// Tanzanian shilling. Tanzania.
	(TZS, 834, 2),
	/// Ukrainian hryvnia. Ukraine.
	(UAH, 980, 2),
	/// Ugandan shilling. Uganda.
	(UGX, 800, 0),
	/// United States dollar. United States, American Samoa, British Indian Ocean Territory, British Virgin Islands, Bonaire, Sint Eustatius and Saba, Ecuador, El Salvador, Guam, Marshall Islands, Federated States of Micronesia, Northern Mariana Islands, Palau, Panama, Puerto Rico, Timor-Leste, Turks and Caicos Islands, U.S. Virgin Islands, United States Minor Outlying Islands.
	(USD, 840, 2),
	/// United States dollar - Next day. United States.
	(USN, 997, 2),
	/// Uruguay Peso en Unidades Indexadas. Uruguay.
	(UYI, 940, 0),
	/// Uruguayan peso. Uruguay.
	(UYU, 858, 2),
	/// Unidad previsional. Uruguay.
	(UYW, 927, 4),
	/// Uzbekistani sum. Uzbekistan.
	(UZS, 860, 2),
	/// Venezuelan digital bolívar. Venezuela.
	(VED, 926, 2),
	/// Venezuelan sovereign bolívar. Venezuela.
	(VES, 928, 2),
	/// Vietnamese đồng. Vietnam.
	(VND, 704, 0),
	/// Vanuatu vatu. Vanuatu.
	(VUV, 548, 0),
	/// Samoan tala. Samoa.
	(WST, 882, 2),
	/// CFA franc BEAC. Cameroon, Central African Republic, Republic of the Congo, Chad, Equatorial Guinea, Gabon.
	(XAF, 950, 0),
	/// Silver (one troy ounce).
	(XAG, 961, 0),
	/// Gold (one troy ounce).
	(XAU, 959, 0),
	/// European Composite Unit.
	(XBA, 955, 0),
	/// European Monetary Unit.
	(XBB, 956, 0),
	/// European Unit of Account 9.
	(XBC, 957, 0),
	/// European Unit of Account 17.
	(XBD, 958, 0),
	/// East Caribbean dollar. Anguilla, Antigua and Barbuda, Dominica, Grenada, Montserrat, Saint Kitts and Nevis, Saint Lucia, Saint Vincent and the Grenadines.
	(XCD, 951, 2),
	/// Special drawing rights. International Monetary Fund.
	(XDR, 960, 0),
	/// CFA franc BCEAO. Benin, Burkina Faso, Ivory Coast, Guinea-Bissau, Mali, Niger, Senegal, Togo.
	(XOF, 952, 0),
	/// Palladium (one troy ounce).
	(XPD, 964, 0),
	/// CFP franc. French Polynesia, New Caledonia, Wallis and Futuna.
	(XPF, 953, 0),
	/// Platinum (one troy ounce).
	(XPT, 962, 0),
	/// SUCRE. Unified System for Regional Compensation.
	(XSU, 994, 0),
	/// Reserved for testing.
	(XTS, 963, 0),
	/// ADB Unit of Account. African Development Bank.
	(XUA, 965, 0),
	/// No currency.
	(XXX, 999, 0),
	/// Yemeni rial. Yemen.
	(YER, 886, 2),
	/// South African rand. South Africa, Eswatini, Lesotho, Namibia.
	(ZAR, 710, 2),
	/// Zambian kwacha. Zambia.
	(ZMW, 967, 2),
	/// Zimbabwe Gold. Zimbabwe.
	(ZWG, 924, 2)
);
