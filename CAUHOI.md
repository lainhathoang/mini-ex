**1. Bạn đã từng dùng AI để hoàn thành những công việc gì?**

- Brainstorm và review giải pháp kỹ thuật.
- Tìm thêm hướng tiếp cận khi gặp vấn đề mới.
- Review code, phát hiện case thiếu hoặc logic chưa hợp lý.
- Hỗ trợ refactor code, viết test case, viết tài liệu ngắn.
- Tóm tắt requirements hoặc chuyển requirements thành checklist triển khai.

**2. Toàn bộ quy trình sử dụng AI trong quá trình làm việc (càng chi tiết càng tốt), khi gặp vấn đề thì xử lý như thế nào?**

Với những vấn đề đơn giản hoặc đã quen thuộc, tôi thường tự nghĩ hướng giải quyết trước, sau đó dùng AI để review lại, gợi ý edge cases, hoặc đề xuất cách viết code/tài liệu rõ ràng hơn.

Với những vấn đề mới, quy trình của tôi thường là:

1. Mô tả rõ bối cảnh cho AI: project đang làm gì, constraint là gì, mục tiêu cần đạt được là gì.
2. Yêu cầu AI đề xuất một hoặc vài hướng giải quyết.
3. Đọc và tìm hiểu lại giải pháp AI đưa ra.
4. Đối chiếu với official docs.
5. Nếu thấy hợp lý thì thử implement ở phạm vi nhỏ.
6. Chạy test.
7. Nếu output của AI sai, quá chung chung hoặc có dấu hiệu hallucination thì tôi sẽ tự Google/read docs, sau đó đưa ngược tài liệu hoặc context chính xác cho AI để nó hỗ trợ tiếp.

**3. Theo bạn cách sử dụng AI nào là tối ưu/best practice?**

Theo tôi, cách dùng AI hiệu quả nhất là xem AI như một trợ lý kỹ thuật. Người dùng AI vẫn cần liên tục bổ sung kiến thức, hiểu rõ hệ thống và nắm được các nguyên lý kỹ thuật để có thể đánh giá, giám sát và chỉnh sửa những gì AI tạo ra.

Một số best practices tôi thường áp dụng là:

- Luôn cung cấp context rõ ràng: requirement, tech stack, folder structure, convention hiện tại.
- Không hỏi quá chung chung.
- Luôn review lại output của AI, đặc biệt là phần business logic, security, database migration và transaction. Có thể cho 2 AI review chéo nhau.
- Ưu tiên yêu cầu AI giải thích trade-off, edge cases, test cases thay vì chỉ sinh code.
- Xác thực bằng official docs, test, compile và code review.
- Developer vẫn cần hiểu overview của hệ thống để biết output nào dùng được, output nào cần sửa hoặc bỏ.
