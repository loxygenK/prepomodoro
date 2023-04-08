macro_rules! messages {
    ( $( $name: ident: $value: literal $( ( $( $arg_name: ident: $arg_type: ty, )+ ) )?; )+ ) => {
        $(
            #[inline(always)]
            pub fn $name($($($arg_name: $arg_type,)*)?) -> String {
                format!($value).trim().to_string()
            }
        )+
    }
}

messages! {
    checklist_ready: "Get ready!";
    checklist_header: "Check list for";
    enter_notify: "To start the work, check boxes using [↑↓/Space] and press [Enter].";
    enter_check: "Do you want to start? There are some items that are not checked yet.";
}

// messages! {
//     checklist_ready: "準備をしましょう!";
//     checklist_header: "確認リスト: ";
//     enter_notify: "[↑↓/Space] でチェックボックスにチェックを入れて、[Enter] を押してください。";
//     enter_check: "チェックされていない項目がありますが、本当に始めますか?";
// }
