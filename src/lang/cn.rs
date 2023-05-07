lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "状态"),
        ("Your Desktop", "你的桌面"),
        ("desk_tip", "你的桌面可以通过下面的 ID 和密码访问。"),
        ("Password", "密码"),
        ("Ready", "就绪"),
        ("Established", "已建立"),
        ("connecting_status", "正在接入 RustDesk 网络..."),
        ("Enable Service", "允许服务"),
        ("Start Service", "启动服务"),
        ("Service is running", "服务正在运行"),
        ("Service is not running", "服务未运行"),
        ("not_ready_status", "未就绪，请检查网络连接"),
        ("Control Remote Desktop", "控制远程桌面"),
        ("Transfer File", "传输文件"),
        ("Connect", "连接"),
        ("Recent Sessions", "最近访问过"),
        ("Address Book", "地址簿"),
        ("Confirmation", "确认"),
        ("TCP Tunneling", "TCP 隧道"),
        ("Remove", "删除"),
        ("Refresh random password", "刷新随机密码"),
        ("Set your own password", "设置密码"),
        ("Enable Keyboard/Mouse", "允许控制键盘/鼠标"),
        ("Enable Clipboard", "允许同步剪贴板"),
        ("Enable File Transfer", "允许传输文件"),
        ("Enable TCP Tunneling", "允许建立 TCP 隧道"),
        ("IP Whitelisting", "IP 白名单"),
        ("ID/Relay Server", "ID/中继服务器"),
        ("Import Server Config", "导入服务器配置"),
        ("Export Server Config", "导出服务器配置"),
        ("Import server configuration successfully", "导入服务器配置信息成功"),
        ("Export server configuration successfully", "导出服务器配置信息成功"),
        ("Invalid server configuration", "服务器配置无效，请修改后重新复制配置信息到剪贴板，然后点击此按钮"),
        ("Clipboard is empty", "复制配置信息到剪贴板后点击此按钮，可以自动导入配置"),
        ("Stop service", "停止服务"),
        ("Change ID", "更改 ID"),
        ("Your new ID", "你的新 ID"),
        ("length %min% to %max%", "长度在 %min% 与 %max% 之间"),
        ("starts with a letter", "以字母开头"),
        ("allowed characters", "使用允许的字符"),
        ("id_change_tip", "只可以使用字母 a-z, A-Z, 0-9, _ (下划线)。首字母必须是 a-z, A-Z。长度在 6 与 16 之间。"),
        ("Website", "网站"),
        ("About", "关于"),
        ("Slogan_tip", ""),
        ("Privacy Statement", "隐私声明"),
        ("Mute", "静音"),
        ("Build Date", "构建日期"),
        ("Version", "版本"),
        ("Home", "主页"),
        ("Audio Input", "音频输入"),
        ("Enhancements", "增强功能"),
        ("Hardware Codec", "硬件编解码"),
        ("Adaptive Bitrate", "自适应码率"),
        ("ID Server", "ID 服务器"),
        ("Relay Server", "中继服务器"),
        ("API Server", "API 服务器"),
        ("invalid_http", "必须以 http:// 或者 https:// 开头"),
        ("Invalid IP", "无效 IP"),
        ("Invalid format", "无效格式"),
        ("server_not_support", "服务器暂不支持"),
        ("Not available", "不可用"),
        ("Too frequent", "修改太频繁，请稍后再试"),
        ("Cancel", "取消"),
        ("Skip", "跳过"),
        ("Close", "关闭"),
        ("Retry", "再试"),
        ("OK", "确认"),
        ("Password Required", "需要密码"),
        ("Please enter your password", "请输入密码"),
        ("Remember password", "记住密码"),
        ("Wrong Password", "密码错误"),
        ("Do you want to enter again?", "是否要再次输入？"),
        ("Connection Error", "连接错误"),
        ("Error", "错误"),
        ("Reset by the peer", "连接被对方关闭"),
        ("Connecting...", "正在连接..."),
        ("Connection in progress. Please wait.", "正在进行连接，请稍候。"),
        ("Please try 1 minute later", "一分钟后再试"),
        ("Login Error", "登录错误"),
        ("Successful", "成功"),
        ("Connected, waiting for image...", "已连接，等待画面传输..."),
        ("Name", "名称"),
        ("Type", "类型"),
        ("Modified", "修改时间"),
        ("Size", "大小"),
        ("Show Hidden Files", "显示隐藏文件"),
        ("Receive", "接受"),
        ("Send", "发送"),
        ("Refresh File", "刷新文件"),
        ("Local", "本地"),
        ("Remote", "远程"),
        ("Remote Computer", "远程电脑"),
        ("Local Computer", "本地电脑"),
        ("Confirm Delete", "确认删除"),
        ("Delete", "删除"),
        ("Properties", "属性"),
        ("Multi Select", "多选"),
        ("Select All", "全选"),
        ("Unselect All", "取消全选"),
        ("Empty Directory", "空文件夹"),
        ("Not an empty directory", "这不是一个空文件夹"),
        ("Are you sure you want to delete this file?", "是否删除此文件？"),
        ("Are you sure you want to delete this empty directory?", "是否删除此空文件夹？"),
        ("Are you sure you want to delete the file of this directory?", "是否删除此文件夹下的文件？"),
        ("Do this for all conflicts", "应用于其它冲突"),
        ("This is irreversible!", "此操作不可逆！"),
        ("Deleting", "正在删除"),
        ("files", "文件"),
        ("Waiting", "正在等待..."),
        ("Finished", "完成"),
        ("Speed", "速度"),
        ("Custom Image Quality", "设置画面质量"),
        ("Privacy mode", "隐私模式"),
        ("Block user input", "阻止用户输入"),
        ("Unblock user input", "取消阻止用户输入"),
        ("Adjust Window", "调节窗口"),
        ("Original", "原始比例"),
        ("Shrink", "收缩"),
        ("Stretch", "伸展"),
        ("Scrollbar", "滚动条"),
        ("ScrollAuto", "自动滚动"),
        ("Good image quality", "画质最优化"),
        ("Balanced", "平衡"),
        ("Optimize reaction time", "速度最优化"),
        ("Custom", "自定义"),
        ("Show remote cursor", "显示远程光标"),
        ("Show quality monitor", "显示质量监测"),
        ("Disable clipboard", "禁用剪贴板"),
        ("Lock after session end", "会话结束后锁定远程电脑"),
        ("Insert", "插入"),
        ("Insert Lock", "锁定远程电脑"),
        ("Refresh", "刷新画面"),
        ("ID does not exist", "ID 不存在"),
        ("Failed to connect to rendezvous server", "连接注册服务器失败"),
        ("Please try later", "请稍后再试"),
        ("Remote desktop is offline", "远程电脑处于离线状态"),
        ("Key mismatch", "Key 不匹配"),
        ("Timeout", "连接超时"),
        ("Failed to connect to relay server", "无法连接到中继服务器"),
        ("Failed to connect via rendezvous server", "无法通过注册服务器建立连接"),
        ("Failed to connect via relay server", "无法通过中继服务器建立连接"),
        ("Failed to make direct connection to remote desktop", "无法直接连接到远程桌面"),
        ("Set Password", "设置密码"),
        ("OS Password", "操作系统密码"),
        ("install_tip", "你正在运行未安装版本，由于 UAC 限制，作为被控端，会在某些情况下无法控制鼠标键盘，或者录制屏幕，请点击下面的按钮将 RustDesk 安装到系统，从而规避上述问题。"),
        ("Click to upgrade", "点击这里升级"),
        ("Click to download", "点击这里下载"),
        ("Click to update", "点击这里更新"),
        ("Configure", "配置"),
        ("config_acc", "为了能够远程控制你的桌面, 请给予 RustDesk \"辅助功能\" 权限。"),
        ("config_screen", "为了能够远程访问你的桌面, 请给予 RustDesk \"屏幕录制\" 权限。"),
        ("Installing ...", "安装中..."),
        ("Install", "安装"),
        ("Installation", "安装"),
        ("Installation Path", "安装路径"),
        ("Create start menu shortcuts", "创建启动菜单快捷方式"),
        ("Create desktop icon", "创建桌面图标"),
        ("agreement_tip", "开始安装即表示接受许可协议。"),
        ("Accept and Install", "同意并安装"),
        ("End-user license agreement", "用户协议"),
        ("Generating ...", "正在生成..."),
        ("Your installation is lower version.", "你安装的版本比当前运行的低。"),
        ("not_close_tcp_tip", "请在使用隧道的时候，不要关闭本窗口"),
        ("Listening ...", "正在等待隧道连接..."),
        ("Remote Host", "远程主机"),
        ("Remote Port", "远程端口"),
        ("Action", "动作"),
        ("Add", "添加"),
        ("Local Port", "本地端口"),
        ("Local Address", "当前地址"),
        ("Change Local Port", "修改本地端口"),
        ("setup_server_tip", "如果需要更快连接速度，你可以选择自建服务器"),
        ("Too short, at least 6 characters.", "太短了，至少 6 个字符"),
        ("The confirmation is not identical.", "两次输入不匹配"),
        ("Permissions", "权限"),
        ("Accept", "接受"),
        ("Dismiss", "拒绝"),
        ("Disconnect", "断开连接"),
        ("Allow using keyboard and mouse", "允许使用键盘鼠标"),
        ("Allow using clipboard", "允许使用剪贴板"),
        ("Allow hearing sound", "允许听到声音"),
        ("Allow file copy and paste", "允许复制粘贴文件"),
        ("Connected", "已连接"),
        ("Direct and encrypted connection", "加密直连"),
        ("Relayed and encrypted connection", "加密中继连接"),
        ("Direct and unencrypted connection", "非加密直连"),
        ("Relayed and unencrypted connection", "非加密中继连接"),
        ("Enter Remote ID", "输入对方 ID"),
        ("Enter your password", "输入密码"),
        ("Logging in...", "正在登录..."),
        ("Enable RDP session sharing", "允许 RDP 会话共享"),
        ("Auto Login", "自动登录（设置断开后锁定才有效）"),
        ("Enable Direct IP Access", "允许 IP 直接访问"),
        ("Rename", "重命名"),
        ("Space", "空格"),
        ("Create Desktop Shortcut", "创建桌面快捷方式"),
        ("Change Path", "更改路径"),
        ("Create Folder", "创建文件夹"),
        ("Please enter the folder name", "请输入文件夹名称"),
        ("Fix it", "修复"),
        ("Warning", "警告"),
        ("Login screen using Wayland is not supported", "不支持使用 Wayland 登录界面"),
        ("Reboot required", "重启后才能生效"),
        ("Unsupported display server", "不支持当前显示服务器"),
        ("x11 expected", "请切换到 x11"),
        ("Port", "端口"),
        ("Settings", "设置"),
        ("Username", "用户名"),
        ("Invalid port", "无效端口"),
        ("Closed manually by the peer", "被对方手动关闭"),
        ("Enable remote configuration modification", "允许远程修改配置"),
        ("Run without install", "不安装直接运行"),
        ("Connect via relay", "中继连接"),
        ("Always connect via relay", "强制走中继连接"),
        ("whitelist_tip", "只有白名单里的 IP 才能访问本机"),
        ("Login", "登录"),
        ("Verify", "验证"),
        ("Remember me", "记住我"),
        ("Trust this device", "信任此设备"),
        ("Verification code", "验证码"),
        ("verification_tip", "检测到新设备登录，已向注册邮箱发送了登录验证码，请输入验证码继续登录"),
        ("Logout", "登出"),
        ("Tags", "标签"),
        ("Search ID", "查找 ID"),
        ("whitelist_sep", "可以使用逗号，分号，空格或者换行符作为分隔符"),
        ("Add ID", "增加 ID"),
        ("Add Tag", "增加标签"),
        ("Unselect all tags", "取消选择所有标签"),
        ("Network error", "网络错误"),
        ("Username missed", "用户名没有填写"),
        ("Password missed", "密码没有填写"),
        ("Wrong credentials", "提供的登录信息错误"),
        ("Edit Tag", "修改标签"),
        ("Unremember Password", "忘记密码"),
        ("Favorites", "收藏"),
        ("Add to Favorites", "加入到收藏"),
        ("Remove from Favorites", "从收藏中删除"),
        ("Empty", "空空如也"),
        ("Invalid folder name", "无效文件夹名称"),
        ("Socks5 Proxy", "Socks5 代理"),
        ("Hostname", "主机名"),
        ("Discovered", "已发现"),
        ("install_daemon_tip", "为了开机启动，请安装系统服务。"),
        ("Remote ID", "远程 ID"),
        ("Paste", "粘贴"),
        ("Paste here?", "粘贴到这里？"),
        ("Are you sure to close the connection?", "是否确认关闭连接？"),
        ("Download new version", "下载新版本"),
        ("Touch mode", "触屏模式"),
        ("Mouse mode", "鼠标模式"),
        ("One-Finger Tap", "单指轻触"),
        ("Left Mouse", "鼠标左键"),
        ("One-Long Tap", "单指长按"),
        ("Two-Finger Tap", "双指轻触"),
        ("Right Mouse", "鼠标右键"),
        ("One-Finger Move", "单指移动"),
        ("Double Tap & Move", "双击并移动"),
        ("Mouse Drag", "鼠标选中拖动"),
        ("Three-Finger vertically", "三指垂直滑动"),
        ("Mouse Wheel", "鼠标滚轮"),
        ("Two-Finger Move", "双指移动"),
        ("Canvas Move", "移动画布"),
        ("Pinch to Zoom", "双指缩放"),
        ("Canvas Zoom", "缩放画布"),
        ("Reset canvas", "重置画布"),
        ("No permission of file transfer", "没有文件传输权限"),
        ("Note", "备注"),
        ("Connection", "连接"),
        ("Share Screen", "共享屏幕"),
        ("Chat", "聊天消息"),
        ("Total", "总计"),
        ("items", "个项目"),
        ("Selected", "已选择"),
        ("Screen Capture", "屏幕录制"),
        ("Input Control", "输入控制"),
        ("Audio Capture", "音频录制"),
        ("File Connection", "文件连接"),
        ("Screen Connection", "屏幕连接"),
        ("Do you accept?", "是否接受？"),
        ("Open System Setting", "打开系统设置"),
        ("How to get Android input permission?", "如何获取安卓的输入权限？"),
        ("android_input_permission_tip1", "为了让远程设备通过鼠标或触屏控制您的安卓设备，你需要允许 RustDesk 使用\"无障碍\"服务。"),
        ("android_input_permission_tip2", "请在接下来的系统设置页面里，找到并进入 [已安装的服务] 页面，将 [RustDesk Input] 服务开启。"),
        ("android_new_connection_tip", "收到新的连接控制请求，对方想要控制你当前的设备。"),
        ("android_service_will_start_tip", "开启录屏权限将自动开启服务，允许其他设备向此设备请求建立连接。"),
        ("android_stop_service_tip", "关闭服务将自动关闭所有已建立的连接。"),
        ("android_version_audio_tip", "当前安卓版本不支持音频录制，请升级至安卓 10 或更高。"),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", "对于已建立的连接，权限可能不会立即发生改变，除非重新建立连接。"),
        ("Account", "账户"),
        ("Overwrite", "覆盖"),
        ("This file exists, skip or overwrite this file?", "这个文件/文件夹已存在，跳过/覆盖？"),
        ("Quit", "退出"),
        ("doc_mac_permission", "https://rustdesk.com/docs/zh-cn/manual/mac#%E5%90%AF%E7%94%A8%E6%9D%83%E9%99%90"),
        ("Help", "帮助"),
        ("Failed", "失败"),
        ("Succeeded", "成功"),
        ("Someone turns on privacy mode, exit", "其他用户使用隐私模式，退出"),
        ("Unsupported", "不支持"),
        ("Peer denied", "被控端拒绝"),
        ("Please install plugins", "请安装插件"),
        ("Peer exit", "被控端退出"),
        ("Failed to turn off", "退出失败"),
        ("Turned off", "退出"),
        ("In privacy mode", "进入隐私模式"),
        ("Out privacy mode", "退出隐私模式"),
        ("Language", "语言"),
        ("Keep RustDesk background service", "保持 RustDesk 后台服务"),
        ("Ignore Battery Optimizations", "忽略电池优化"),
        ("android_open_battery_optimizations_tip", "如需关闭此功能，请在接下来的 RustDesk 应用设置页面中，找到并进入 [电源] 页面，取消勾选 [不受限制]"),
        ("Start on Boot", "开机自启动"),
        ("Start the screen sharing service on boot, requires special permissions", "开机自动启动屏幕共享服务，此功能需要一些特殊权限。"),
        ("Connection not allowed", "对方不允许连接"),
        ("Legacy mode", "传统模式"),
        ("Map mode", "1：1 传输"),
        ("Translate mode", "翻译模式"),
        ("Use permanent password", "使用固定密码"),
        ("Use both passwords", "同时使用两种密码"),
        ("Set permanent password", "设置固定密码"),
        ("Enable Remote Restart", "允许远程重启"),
        ("Allow remote restart", "允许远程重启"),
        ("Restart Remote Device", "重启远程电脑"),
        ("Are you sure you want to restart", "确定要重启"),
        ("Restarting Remote Device", "正在重启远程设备"),
        ("remote_restarting_tip", "远程设备正在重启, 请关闭当前提示框, 并在一段时间后使用永久密码重新连接"),
        ("Copied", "已复制"),
        ("Exit Fullscreen", "退出全屏"),
        ("Fullscreen", "全屏"),
        ("Mobile Actions", "移动端操作"),
        ("Select Monitor", "选择监视器"),
        ("Control Actions", "控制操作"),
        ("Display Settings", "显示设置"),
        ("Ratio", "比例"),
        ("Image Quality", "画质"),
        ("Scroll Style", "滚屏方式"),
        ("Show Menubar", "显示菜单栏"),
        ("Hide Menubar", "隐藏菜单栏"),
        ("Direct Connection", "直接连接"),
        ("Relay Connection", "中继连接"),
        ("Secure Connection", "安全连接"),
        ("Insecure Connection", "非安全连接"),
        ("Scale original", "原始尺寸"),
        ("Scale adaptive", "适应窗口"),
        ("General", "常规"),
        ("Security", "安全"),
        ("Theme", "主题"),
        ("Dark Theme", "暗黑主题"),
        ("Light Theme", "明亮主题"),
        ("Dark", "黑暗"),
        ("Light", "明亮"),
        ("Follow System", "跟随系统"),
        ("Enable hardware codec", "使用硬件编解码"),
        ("Unlock Security Settings", "解锁安全设置"),
        ("Enable Audio", "允许传输音频"),
        ("Unlock Network Settings", "解锁网络设置"),
        ("Server", "服务器"),
        ("Direct IP Access", "IP 直接访问"),
        ("Proxy", "代理"),
        ("Apply", "应用"),
        ("Disconnect all devices?", "断开所有远程连接？"),
        ("Clear", "清空"),
        ("Audio Input Device", "音频输入设备"),
        ("Deny remote access", "拒绝远程访问"),
        ("Use IP Whitelisting", "只允许白名单上的 IP 访问"),
        ("Network", "网络"),
        ("Enable RDP", "允许 RDP 访问"),
        ("Pin menubar", "固定菜单栏"),
        ("Unpin menubar", "取消固定菜单栏"),
        ("Recording", "录屏"),
        ("Directory", "目录"),
        ("Automatically record incoming sessions", "自动录制来访会话"),
        ("Change", "更改"),
        ("Start session recording", "开始录屏"),
        ("Stop session recording", "结束录屏"),
        ("Enable Recording Session", "允许录制会话"),
        ("Allow recording session", "允许录制会话"),
        ("Enable LAN Discovery", "允许局域网发现"),
        ("Deny LAN Discovery", "拒绝局域网发现"),
        ("Write a message", "输入聊天消息"),
        ("Prompt", "提示"),
        ("Please wait for confirmation of UAC...", "请等待对方确认 UAC..."),
        ("elevated_foreground_window_tip", "远端桌面的当前窗口需要更高的权限才能操作, 暂时无法使用鼠标键盘, 可以请求对方最小化当前窗口, 或者在连接管理窗口点击提升。为避免这个问题，建议在远端设备上安装本软件。"),
        ("Disconnected", "会话已结束"),
        ("Other", "其他"),
        ("Confirm before closing multiple tabs", "关闭多个标签页时向您确认"),
        ("Keyboard Settings", "键盘设置"),
        ("Full Access", "完全访问"),
        ("Screen Share", "仅共享屏幕"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland 需要 Ubuntu 21.04 或更高版本。"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland 需要更高版本的 linux 发行版。 请尝试 X11 桌面或更改您的操作系统。"),
        ("JumpLink", "查看"),
        ("Please Select the screen to be shared(Operate on the peer side).", "请选择要分享的画面（对端操作）。"),
        ("Show RustDesk", "显示 RustDesk"),
        ("This PC", "此电脑"),
        ("or", "或"),
        ("Continue with", "使用"),
        ("Elevate", "提权"),
        ("Zoom cursor", "缩放光标"),
        ("Accept sessions via password", "只允许密码访问"),
        ("Accept sessions via click", "只允许点击访问"),
        ("Accept sessions via both", "允许密码或点击访问"),
        ("Please wait for the remote side to accept your session request...", "请等待对方接受你的连接..."),
        ("One-time Password", "一次性密码"),
        ("Use one-time password", "使用一次性密码"),
        ("One-time password length", "一次性密码长度"),
        ("Request access to your device", "请求访问你的设备"),
        ("Hide connection management window", "隐藏连接管理窗口"),
        ("hide_cm_tip", "在只允许密码连接并且只用固定密码的情况下才允许隐藏"),
        ("wayland_experiment_tip", "Wayland 支持处于实验阶段，如果你需要使用无人值守访问，请使用 X11。"),
        ("Right click to select tabs", "右键选择选项卡"),
        ("Skipped", "已跳过"),
        ("Add to Address Book", "添加到地址簿"),
        ("Group", "小组"),
        ("Search", "搜索"),
        ("Closed manually by web console", "被 web 控制台手动关闭"),
        ("Local keyboard type", "本地键盘类型"),
        ("Select local keyboard type", "请选择本地键盘类型"),
        ("software_render_tip", "如果你使用英伟达显卡, 并且远程窗口在会话建立后会立刻关闭, 那么安装 nouveau 驱动并且选择使用软件渲染可能会有帮助。重启软件后生效。"),
        ("Always use software rendering", "始终使用软件渲染"),
        ("config_input", "为了能够通过键盘控制远程桌面, 请给予 RustDesk \"输入监控\" 权限。"),
        ("config_microphone", "为了支持通过麦克风进行音频传输，请给予 RustDesk \"录音\"权限。"),
        ("request_elevation_tip", "如果对面有人, 也可以请求提升权限。"),
        ("Wait", "等待"),
        ("Elevation Error", "提权失败"),
        ("Ask the remote user for authentication", "请求远端用户授权"),
        ("Choose this if the remote account is administrator", "当对面电脑是管理员账号时选择该选项"),
        ("Transmit the username and password of administrator", "发送管理员账号的用户名密码"),
        ("still_click_uac_tip", "依然需要被控端用户在运行 RustDesk 的 UAC 窗口点击确认。"),
        ("Request Elevation", "请求提权"),
        ("wait_accept_uac_tip", "请等待远端用户确认 UAC 对话框。"),
        ("Elevate successfully", "提权成功"),
        ("uppercase", "大写字母"),
        ("lowercase", "小写字母"),
        ("digit", "数字"),
        ("special character", "特殊字符"),
        ("length>=8", "长度不小于 8"),
        ("Weak", "弱"),
        ("Medium", "中"),
        ("Strong", "强"),
        ("Switch Sides", "反转访问方向"),
        ("Please confirm if you want to share your desktop?", "请确认是否要让对方访问你的桌面？"),
        ("Display", "显示"),
        ("Default View Style", "默认显示方式"),
        ("Default Scroll Style", "默认滚动方式"),
        ("Default Image Quality", "默认图像质量"),
        ("Default Codec", "默认编解码"),
        ("Bitrate", "码率"),
        ("FPS", "帧率"),
        ("Auto", "自动"),
        ("Other Default Options", "其它默认选项"),
        ("Voice call", "语音通话"),
        ("Text chat", "文字聊天"),
        ("Stop voice call", "停止语音通话"),
        ("relay_hint_tip", "可能无法直连，可以尝试中继连接。\n另外，如果想直接使用中继连接，可以在 ID 后面添加/r，或者在卡片选项里选择强制走中继连接。"),
        ("Reconnect", "重连"),
        ("Codec", "编解码"),
        ("Resolution", "分辨率"),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("idd_driver_tip", "安装虚拟显示器驱动，以便在没有连接显示器的情况下启动虚拟显示器进行控制。"),
        ("confirm_idd_driver_tip", "安装虚拟显示器驱动的选项已勾选。请注意，测试证书将被安装以信任虚拟显示器驱动。测试证书仅会用于信任Rustdesk的驱动。"),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", "此文件与对方的一致"),
        ("show_monitors_tip", ""),
        ("View Mode", "浏览模式"),
        ("login_linux_tip", "登录被控端的 Linux 账户，才能启用 X 桌面"),
        ("verify_rustdesk_password_tip", "验证 RustDesk 密码"),
        ("remember_account_tip", "记住此账户"),
        ("os_account_desk_tip", "在无显示器的环境下，此账户用于登录被控系统，并启用桌面"),
        ("OS Account", "系统账户"),
        ("another_user_login_title_tip", "其他用户已登录"),
        ("another_user_login_text_tip", "断开"),
        ("xorg_not_found_title_tip", "Xorg 未安装"),
        ("xorg_not_found_text_tip", "请安装 Xorg"),
        ("no_desktop_title_tip", "desktop 未安装"),
        ("no_desktop_text_tip", "请安装 desktop"),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", "指纹"),
        ("Copy Fingerprint", "复制指纹"),
        ("no fingerprints", "没有指纹"),
    ].iter().cloned().collect();
}