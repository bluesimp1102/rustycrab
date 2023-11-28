Permission-missing = Thiếu quyền sử dụng lệnh

# Lỗi thông thường
commonError-noUserInfo = Không thể tìm thấy thông tin { $user }.
commonError-noBotGuildConfig = Không tìm thấy cấu hình bang hội.
commonError-noTag = Gắn thẻ ít nhất một người.
commonError-tag1 = Gắn thẻ một người

# Utilities
interaction-denied = Bạn không đủ quyền.
requested-user = Được yêu cầu bởi @{ $username }

# Commands 
command-error = Đã có lỗi xảy ra khi xử lí lệnh.
command-guildonly = Lệnh chỉ được sử dụng ở server.

# Lệnh của chủ sở hữu Bot
command-botowner = Quản lý quyền của BotOwner
command-botowner-addSuccess = Đã thêm quyền BotOwner cho { $user }
command-botowner-removeSuccess = Đã xóa quyền của BotOwner { $user }
command-botowner-alreadyOwner = { $user } đã có quyền BotOwner
command-botowner-notOwner = { $user } không có quyền BotOwner.
command-botowner-listEmpty = trống...
command-botowner-listTitle = Người dùng có quyền BotOwner

# Lệnh quản trị Bot
command-botadmin = Quản lý quyền BotAdmin
command-botadmin-addSuccess = Đã thêm quyền BotAdmin cho { $user }
command-botadmin-removeSuccess = Đã xóa quyền của BotAdmin { $user }
command-botadmin-alreadyAdmin = { $user } đã có quyền BotAdmin
command-botadmin-notAdmin = { $user } không có quyền BotAdmin.
command-botadmin-listEmpty = trống...
command-botadmin-listTitle = Người dùng có quyền BotAdmin

# Lệnh ngôn ngữ
command-language = Thay đổi ngôn ngữ của bot
command-language-invalidLocale = Ngôn ngữ không hợp lệ. Ngôn ngữ có sẵn: { $locales }
command-language-localeChanged = Ngôn ngữ của bot đã chuyển sang `{ $locale }`

# Lệnh Avatar
command-avatar = Kiểm tra hình đại diện của người dùng

# Lệnh toán
command-math = Tính các phép toán cơ bản
command-math-invalid = Phép học không hợp lệ

# Lệnh prefix
command-prefix = Thay đổi prefix máy chủ của bot
command-prefix-invalid = prefix không hợp lệ, prefix không được để trống
command-prefix-success = prefix được cập nhật thành công. Prefix: `{ $prefix }`

# Lệnh check banner
command-banner = Kiểm tra banner người dùng

# Lệnh giúp đỡ
command-help = hiển thị bot hoặc lệnh trợ giúp

# Lệnh ping
command-ping = Kiểm tra thời gian phản hồi trung bình của API và Bot

# Lệnh Snipe
command-snipe = Snipe các tin nhắn đã xóa
command-snipe-invalid-position = Không tìm thấy tin nhắn đã xóa tại vị trí được chỉ định.

# Lệnh Ban
command-ban = Ban một hoặc nhiều member
command-ban-success = Đã ban { $user } khỏi server.
command-ban-fail = Xảy ra lỗi khi ban { $user } khỏi server 
command-ban-admin = Không thể ban { $user } do người dùng có quyền admin.

# Lệnh Unban
command-unban = Unban người dùng
command-unban-success=Người dùng không bị ban { $user }
command-unban-fail = Đã xảy ra lỗi khi cố gắng unban { $user }
command-unban-notfound= Người dùng { $user } không bị ban

# Lệnh Kick
command-kick = Đá một hoặc nhiều thành viên
command-kick-success=Đã đá { $user } ra khỏi máy chủ.
command-kick-fail = Đã xảy ra lỗi khi cố gắng kick { $user } khỏi máy chủ.
command-kick-admin = Không thể kick người dùng { $user } với quyền Admin

# Lệnh Timeout
command-timeout = Mute một hoặc nhiều thành viên
command-timeout-success = Đã mute { $user } trong { $duration } giây.
command-timeout-fail = Đã xảy ra lỗi khi cố gắng mute { $user } trong { $duration } giây.
command-timeout-admin = Không thể mute người dùng { $user } với quyền Admin

# Lệnh Untimeout
command-untimeout = Unmute một hoặc nhiều thành viên
command-untimeout-success = Đã unmute { $user }.
command-untimeout-fail = Đã xảy ra lỗi khi cố unmute { $user }.
command-untimeout-notfound = { $user } hiện không bị mute.

# Role Command
command-role = Thêm hoặc bỏ role được chỉ định từ một hoặc nhiều thành viên
command-role-add-success = Đã thêm role { $role } cho { $user } thành công.
command-role-add-failed = Không thể thêm role { $role } cho { $user }. Lỗi: { $err }
command-role-remove-success = Đã bỏ role { $role } từ { $user } thành công.
command-role-remove-failed = Không thể bỏ role { $role } từ { $user }. Lỗi: { $err }
command-role-no-perm = Bot không có quyền quản lý role { $role }.

# AFK
command-afk = Set trạng thái AFK
command-afk-success = { $user }, trạng thái AFK của bạn đã được cập nhật.
afk-notification = [Server { $server }] Thông báo AFK: { $user } đã quay lại.
afk-is-afk = { $user }, `@{ $afk_name }` đang ở trạng thái AFK { $since } { $message }
afk-notifyme = Notify me
afk-is-back = { $user } không còn ở trạng thái AFK nữa
afk-notify-added = Bạn sẽ nhận được thông báo khi người dùng này trở lại.
afk-notfound = Người dùng này không còn ở trạng thái AFK nữa.

# VOICE & MUSIC
music-note = Tính năng này chỉ chấp nhận URL của Youtube (bài hát và danh sách phát) và URL bản nhạc đơn từ SoundCloud. Kết quả tìm kiếm sẽ lấy video đầu tiên từ tìm kiếm Youtube và thêm vào hàng đợi.
music-not-same-channel = Bạn cần ở trong cùng kênh voice với bot để sử dụng lệnh này.
music-not-playing = Tớ đang không phát nhạc ở kênh nào cả.
music-no-voice = Tớ đang không ở trong kênh voice nào.
music-user-novoice = Bạn cần ở trong kênh voice để sử dụng lệnh.
music-cannot-connect = Tớ không thể kết nối với kênh của bạn.
music-error-track = Lỗi khi truy xuất thông tin bài hát.
music-duration = Thời lượng
music-position-inqueue = Vị trí trong hàng đợi
music-playlist-fetch-error = Lỗi khi lấy bài hát từ danh sách phát, vui lòng kiểm tra lại link hoặc tính khả dụng của video/danh sách phát.
music-nowplaying = Đang phát
# Join Command
command-join = Yêu cầu bot tham gia kênh voice.
command-join-nochannel = Bạn cần ở trong kênh voice để sử dụng lệnh.
command-join-joined = Đã tham gia { $channel }
command-join-failed = Không thể tham gia { $channel }! Lỗi: { $err }

# Leave Command
command-leave = Yêu cầu bot rời kênh voice.
command-leave-left = Đã rời kênh voice.
command-leave-failed = Không thể rời kênh voice.

# Loop Command
command-loop = Lặp lại bài nhạc hoặc danh sách nhạc
command-loop-track = Đang lặp bài hát hiện tại.
command-loop-track-failed = Không thể lặp bài hát hiện tại.
command-loop-queue = Lặp toàn bộ hàng đợi.
command-loop-invalid =  Invalid loop type. accepts: current/one/all/queue

# Pause Command
command-pause = Tạm dừng bài hát đang phát.
command-pause-paused = Đã tạm dừng bài hát.
command-pause-unpaused = Đã tiếp tục phát bài hát.

# Play Command
command-play = Phát nhạc.
command-play-invalid-url = Vui lòng cung cấp URL hoặc truy vấn tìm kiếm hợp lệ để phát.
command-play-added-tracks = Đã thêm { $count } bài hát vào hàng đợi.
command-play-added-track = Đã thêm bài hát.

# Queue
music-queue-empty = Hàng đợi trống.
music-queue-title = Hàng đợi: { $count } bài hát.
music-queue-prev = Trang Trước
music-queue-next = Trang Sau