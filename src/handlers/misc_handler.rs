pub struct MiscManager;
use console::style;

impl MiscManager {
    pub fn help_p10k_script_generation(&self) -> anyhow::Result<()> {
        let script = String::from(
            "function prompt_rchore() {
    result=$(rchore tasks stats -s)
    if [[ \"$result\" == \"logged_out\" ]]
    then
        p10k segment -i \' \' -f yellow -t \"logged out of rChore\"
    else
        header=$(cut -d'_' -f1 <<<\"$result\")
        completed=$(cut -d'_' -f2 <<<\"$result\")
        total=$(cut -d'_' -f4 <<<\"$result\")
        if (( $(echo \"scale=1; $total == 0\" | bc -l) ))
        then
        p10k segment -i ' ' -b green -f black -t \"$header: No tasks!\"
        return
        fi
        if (( $(echo \"scale=1; $completed/$total > 0.7\" | bc -l) ))
        then
        p10k segment -i ' ' -b green -f black -t \"$header: $completed/$total\"
        elif (( $(echo \"scale=1; $completed/$total >= 0.5\" | bc -l) ))
        then
        p10k segment -i ' ' -b yellow -f black -t \"$header: $completed/$total\"
        else
        p10k segment -i ' ' -b red -f yellow -t \"$header: $completed/$total\"
        fi
    fi
}",
        );
        println!(
            "{}\n\n        {}\n        {}\n        {}\n\n{}\n\n{}",
            style("Steps to add rchore p10k battery:").bold().underlined().green(),
            style("* Open .p10k.zsh file").italic().cyan(),
            style("* Copy the below function and paste it").italic().cyan(),
            style("* Add rchore to either POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS or POWERLEVEL9K_LEFT_PROMPT_ELEMENTS").italic().cyan(),
            style(&script),
            style("If you are facing any issues, feel free to open an issue at https://github.com/DarthBenro008/rchore").italic()
        );
        Ok(())
    }
}
