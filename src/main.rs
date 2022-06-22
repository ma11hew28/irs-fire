use json::JsonValue;
use std::fs::File;
use std::io::Write;

const CR_LF: &str = "\r\n";
const RECORD_LEN: usize = 750;

fn main() {
    let mut args = std::env::args().skip(1);
    let input_filename = args.next().unwrap();
    let output_filename = args.next().unwrap();
    let input = std::fs::read_to_string(input_filename).unwrap();
    let objects = json::parse(&input).unwrap();
    let output_file = File::create(output_filename).unwrap();
    let mut runner = Runner::new(output_file);
    runner.set_fields_and_make_and_write_records(objects);
}

struct Runner {
    output_file: File,
    a_record_total: u32,
    b_record_total: u32,
    record_count: u32,
    payment_year: u16,
    b_record_count: u32,
    control_totals: [i64; 18],
    state_tax_withheld_total: Option<i64>,
    local_tax_withheld_total: Option<i64>,
    cfsf_number: u8,
}

impl Runner {
    fn new(output_file: File) -> Self {
        Self {
            output_file,
            a_record_total: 0,
            b_record_total: 0,
            record_count: 0,
            payment_year: 0,
            b_record_count: 0,
            control_totals: [0; 18],
            state_tax_withheld_total: None,
            local_tax_withheld_total: None,
            cfsf_number: 0,
        }
    }

    fn set_fields_and_make_and_write_records(&mut self, objects: JsonValue) {
        self.a_record_total = count_objects_by_record_type_up_to_max(&objects, "A", 99_000);
        self.b_record_total = count_objects_by_record_type_up_to_max(&objects, "B", 999_996);
        for object in objects.members() {
            self.record_count += 1;
            let record_type = object["recordType"].as_str().unwrap();
            let record = match record_type {
                "T" => {
                    self.payment_year = object["paymentYear"].as_u16().unwrap();
                    self.make_t_record(object)
                }
                "A" => {
                    if self.b_record_count > 0 {
                        self.make_and_write_c_and_k_records_and_update_count();
                        self.b_record_count = 0;
                    }
                    self.make_a_record(object)
                }
                "B" => {
                    self.b_record_count += 1;
                    self.make_b_record(object)
                }
                _ => panic!("Invalid record type"),
            };
            self.write(record);
        }
        self.record_count += 1;
        self.make_and_write_c_and_k_records_and_update_count();
        let f_record = self.make_f_record();
        self.write(f_record);
    }

    fn make_t_record(&self, object: &JsonValue) -> String {
        let record_type = "T";
        let is_prior_year = object["isPriorYear"].as_str().unwrap_or(" ");
        let transmitter = &object["transmitter"];
        let transmitter_tin = transmitter["tin"].as_str().unwrap();
        let transmitter_control_code = transmitter["controlCode"].as_str().unwrap();
        let is_test_file = object["isTestFile"].as_str().unwrap_or(" ");
        let transmitter_is_foreign_entity = transmitter["isForeignEntity"].as_str().unwrap_or(" ");
        let transmitter_name = transmitter["name"].as_str().unwrap_to_upper();
        let transmitter_name_2 = transmitter["name2"].as_str().unwrap_to_upper_or_default();
        let company = &object["company"];
        let company_name = company["name"].as_str().unwrap_to_upper();
        let company_name_2 = company["name2"].as_str().unwrap_to_upper_or_default();
        let company_address = &company["address"];
        let company_address_street = company_address["street"].as_str().unwrap_to_upper();
        let company_address_city = company_address["city"].as_str().unwrap_to_upper();
        let company_address_state = company_address["state"].as_str().unwrap();
        let company_address_zip = company_address["zip"].as_str().unwrap();
        let contact = &object["contact"];
        let contact_name = contact["name"].as_str().unwrap_to_upper();
        let contact_phone = contact["phone"].as_str().unwrap();
        let contact_email = contact["email"].as_str().unwrap();
        let is_vendor = "I";
        let vendor_name = "";
        let vendor_address_street = "";
        let vendor_address_city = "";
        let vendor_address_state = "  ";
        let vendor_address_zip = "";
        let vendor_contact_name = "";
        let vendor_contact_phone = "";
        let vendor_is_foreign_entity = " ";
        let fields = [
            record_type,
            &self.payment_year.to_string(),
            is_prior_year,
            transmitter_tin,
            transmitter_control_code,
            &blanks(7),
            is_test_file,
            transmitter_is_foreign_entity,
            &format!("{:40}", transmitter_name),
            &format!("{:40}", transmitter_name_2),
            &format!("{:40}", company_name),
            &format!("{:40}", company_name_2),
            &format!("{:40}", company_address_street),
            &format!("{:40}", company_address_city),
            company_address_state,
            &format!("{:9}", company_address_zip),
            &blanks(15),
            &format!("{:08}", self.b_record_total),
            &format!("{:40}", contact_name),
            &format!("{:15}", contact_phone),
            &format!("{:50}", contact_email),
            &blanks(91),
            &format!("{:08}", self.record_count),
            &blanks(10),
            is_vendor,
            &format!("{:40}", vendor_name),
            &format!("{:40}", vendor_address_street),
            &format!("{:40}", vendor_address_city),
            vendor_address_state,
            &format!("{:9}", vendor_address_zip),
            &format!("{:40}", vendor_contact_name),
            &format!("{:15}", vendor_contact_phone),
            &blanks(35),
            vendor_is_foreign_entity,
            &blanks(8),
            CR_LF,
        ];
        fields.concat()
    }

    fn make_a_record(&self, object: &JsonValue) -> String {
        let record_type = "A";
        let is_cfsf_program = object["isCfsfProgram"].as_str().unwrap_or(" ");
        let issuer = &object["issuer"];
        let issuer_tin = issuer["tin"].as_str().unwrap();
        let issuer_name_control = issuer["nameControl"].as_str().unwrap_or_default();
        let is_last_filing = object["isLastFiling"].as_str().unwrap_or(" ");
        let return_type = "1"; // 1099-DIV
        let amount_codes = object["amountCodes"].as_str().unwrap();
        let issuer_is_foreign_entity = issuer["isForeignEntity"].as_str().unwrap_or(" ");
        let issuer_name = issuer["name"].as_str().unwrap_to_upper();
        let transfer_agent = &object["transferAgent"];
        let transfer_agent_name = &transfer_agent["name"];
        let (issuer_name_2_val, is_transfer_agent) = if !transfer_agent_name.is_null() {
            (transfer_agent_name, "1")
        } else {
            (&issuer["name2"], "0")
        };
        let issuer_name_2 = issuer_name_2_val.as_str().unwrap_to_upper_or_default();
        let issuer_address = &issuer["address"];
        let issuer_address_street = issuer_address["street"].as_str().unwrap_to_upper();
        let issuer_address_city = issuer_address["city"].as_str().unwrap_to_upper();
        let issuer_address_state = issuer_address["state"].as_str().unwrap();
        let issuer_address_zip = issuer_address["zip"].as_str().unwrap();
        let issuer_phone = issuer["phone"].as_str().unwrap();
        let fields = [
            record_type,
            &self.payment_year.to_string(),
            is_cfsf_program,
            &blanks(5),
            issuer_tin,
            &format!("{:4}", issuer_name_control),
            is_last_filing,
            &format!("{:2}", return_type),
            &format!("{:18}", amount_codes),
            &blanks(6),
            issuer_is_foreign_entity,
            &format!("{:40}", issuer_name),
            &format!("{:40}", issuer_name_2),
            is_transfer_agent,
            &format!("{:40}", issuer_address_street),
            &format!("{:40}", issuer_address_city),
            issuer_address_state,
            &format!("{:9}", issuer_address_zip),
            &format!("{:15}", issuer_phone),
            &blanks(260),
            &format!("{:08}", self.record_count),
            &blanks(241),
            CR_LF,
        ];
        fields.concat()
    }

    fn make_b_record(&mut self, object: &JsonValue) -> String {
        let record_type = "B";
        let is_corrected_return = object["isCorrectedReturn"].as_str().unwrap_or(" ");
        let payee = &object["payee"];
        let payee_name_control = payee["nameControl"].as_str().unwrap_or_default();
        let payee_tin_type = payee["tinType"].as_str().unwrap_or(" ");
        let payee_tin = payee["tin"].as_str().unwrap();
        let account_number = object["accountNumber"].as_str().unwrap_or_default();
        let office_code = object["officeCode"].as_str().unwrap_or_default();
        let mut payment_amounts = String::with_capacity(18 * 12);
        let amounts = &object["paymentAmounts"];
        let keys = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h",
            "j",
        ];
        for (idx, key) in keys.iter().enumerate() {
            let amount = amounts[*key].as_i64().unwrap_or_default();
            self.control_totals[idx] += amount;
            let payment_amount = &format!("{:012}", amount);
            payment_amounts.push_str(payment_amount);
        }
        let payee_name = payee["name"].as_str().unwrap_to_upper();
        let payee_name_2 = payee["name2"].as_str().unwrap_to_upper_or_default();
        let payee_address = &payee["address"];
        let payee_address_is_foreign = payee_address["isForeign"].as_str().unwrap_or(" ");
        let payee_address_street = payee_address["street"].as_str().unwrap_to_upper();
        let payee_address_city = payee_address["city"].as_str().unwrap_to_upper();
        let payee_address_state = payee_address["state"].as_str().unwrap();
        let payee_address_zip = payee_address["zip"].as_str().unwrap();
        // Form 1099-DIV fields
        let is_second_tin_notice = object["isSecondTinNotice"].as_str().unwrap_or(" ");
        let foreign_country_or_us_possession = object["foreignCountryOrUsPossession"]
            .as_str()
            .unwrap_or_default();
        let is_fatca_filing_requirement =
            object["isFatcaFilingRequirement"].as_str().unwrap_or(" ");
        let special_data_entries = object["specialDataEntries"].as_str().unwrap_or_default();
        let state_tax_withheld = if let Some(amount) = object["stateTaxWithheld"].as_i64() {
            if let Some(total) = self.state_tax_withheld_total {
                self.state_tax_withheld_total = Some(total + amount);
            } else {
                self.state_tax_withheld_total = Some(amount);
            }
            format!("{:012}", amount)
        } else {
            blanks(12)
        };
        let local_tax_withheld = if let Some(amount) = object["localTaxWithheld"].as_i64() {
            if let Some(total) = self.local_tax_withheld_total {
                self.local_tax_withheld_total = Some(total + amount);
            } else {
                self.local_tax_withheld_total = Some(amount);
            }
            format!("{:012}", amount)
        } else {
            blanks(12)
        };
        self.cfsf_number = object["cfsfNumber"].as_u8().unwrap_or_default();
        let cfsf_code = if self.cfsf_number != 0 {
            format!("{:02}", self.cfsf_number)
        } else {
            blanks(2)
        };
        let fields = [
            record_type,
            &self.payment_year.to_string(),
            is_corrected_return,
            &format!("{:4}", payee_name_control),
            payee_tin_type,
            payee_tin,
            &format!("{:20}", account_number),
            &format!("{:4}", office_code),
            &blanks(10),
            &payment_amounts,
            &blanks(16),
            payee_address_is_foreign,
            &format!("{:40}", payee_name),
            &format!("{:40}", payee_name_2),
            &format!("{:40}", payee_address_street),
            &blanks(40),
            &format!("{:40}", payee_address_city),
            payee_address_state,
            &format!("{:9}", payee_address_zip),
            &blanks(1),
            &format!("{:08}", self.record_count),
            &blanks(36),
            is_second_tin_notice,
            "  ", // &blanks(2)
            &format!("{:40}", foreign_country_or_us_possession),
            is_fatca_filing_requirement,
            &blanks(75),
            &format!("{:60}", special_data_entries),
            &state_tax_withheld,
            &local_tax_withheld,
            &cfsf_code,
            CR_LF,
        ];
        fields.concat()
    }

    fn make_and_write_c_and_k_records_and_update_count(&mut self) {
        let c_record = self.make_c_record();
        self.write(c_record);
        self.record_count += 1;
        if self.cfsf_number != 0 {
            let k_record = self.make_k_record();
            self.write(k_record);
            self.record_count += 1;
        }
    }

    fn make_c_record(&self) -> String {
        let record_type = "C";
        let fields = [
            record_type,
            &format!("{:08}", self.b_record_count),
            &blanks(6),
            &self.control_totals.map(|t| format!("{:018}", t)).concat(),
            &blanks(160),
            &format!("{:08}", self.record_count),
            &blanks(241),
            CR_LF,
        ];
        fields.concat()
    }

    fn make_k_record(&self) -> String {
        let record_type = "K";
        let state_tax_withheld_total = if let Some(total) = self.state_tax_withheld_total {
            format!("{:018}", total)
        } else {
            blanks(18)
        };
        let local_tax_withheld_total = if let Some(total) = self.local_tax_withheld_total {
            format!("{:018}", total)
        } else {
            blanks(18)
        };
        let fields = [
            record_type,
            &format!("{:08}", self.b_record_count),
            &blanks(6),
            &self.control_totals.map(|t| format!("{:018}", t)).concat(),
            &blanks(160),
            &format!("{:08}", self.record_count),
            &blanks(199),
            &format!("{:018}", state_tax_withheld_total),
            &format!("{:018}", local_tax_withheld_total),
            &blanks(4),
            &format!("{:02}", self.cfsf_number),
            CR_LF,
        ];
        fields.concat()
    }

    fn make_f_record(&self) -> String {
        let record_type = "F";
        let fields = [
            record_type,
            &format!("{:08}", self.a_record_total),
            &zeros(21),
            &blanks(19),
            &format!("{:08}", self.b_record_total),
            &blanks(442),
            &format!("{:08}", self.record_count),
            &blanks(241),
            CR_LF,
        ];
        fields.concat()
    }

    fn write(&mut self, record: String) {
        assert_eq!(record.len(), RECORD_LEN);
        self.output_file.write_all(record.as_bytes()).unwrap();
    }
}

trait Unwrap {
    fn unwrap_to_upper(&self) -> String;
    fn unwrap_to_upper_or_default(&self) -> String;
}

impl Unwrap for Option<&str> {
    fn unwrap_to_upper(&self) -> String {
        self.unwrap().to_ascii_uppercase()
    }

    fn unwrap_to_upper_or_default(&self) -> String {
        match self {
            Some(value) => value.to_ascii_uppercase(),
            None => Default::default(),
        }
    }
}

fn blanks(n: usize) -> String {
    " ".repeat(n)
}

fn zeros(n: usize) -> String {
    "0".repeat(n)
}

fn count_objects_by_record_type_up_to_max(objects: &JsonValue, record_type: &str, max: u32) -> u32 {
    objects.members().fold(0u32, |c, o| {
        if c == max {
            panic!();
        } else if o["recordType"].as_str().unwrap() != record_type {
            c
        } else {
            c + 1
        }
    })
}
