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
    too_small_screen: "[!] The window is too small! Minimum {width}x{height} is required.\n    \
    Please make the available region larger. Timer is going in background!" (width: u16, height: u16,);
}

// messages! {
//     checklist_ready: "準備をしましょう!";
//     checklist_header: "確認リスト: ";
//     enter_notify: "[↑↓/Space] でチェックボックスにチェックを入れて、[Enter] を押してください。";
//     enter_check: "チェックされていない項目がありますが、本当に始めますか?";
//     too_small_screen: "[!] 画面が小さすぎます! {width}x{height} が必要です。\n    \
//     画面を大きくしてください。タイマーは背景で進行中です!" (width: u16, height: u16,);
// }
