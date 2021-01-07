$(function () {
    validateRule();
    $('.imgcode').click(function () {
        var url = "/captchaImg/118000";
        $(".imgcode").attr("src", url);
    });
});

$.validator.setDefaults({
    submitHandler: function () {
        login();
    }
});

function login() {
    $.modal.loading($("#btnSubmit").data("loading"));
    var username = $.common.trim($("input[name='username']").val());
    var password = $.common.trim($("input[name='password']").val());
    var validateCode = $("input[name='validateCode']").val();
    /*var rememberMe = $("input[name='rememberme']").is(':checked');*/
    var loginUser = {
        login_name: username,
        password: password,
        validate_code: validateCode
    }
    $.ajax({
        type: "post",
        url: "/login_in",
        data: JSON.stringify(loginUser),
        headers: {"content-type": "application/json"},
        success: function (r) {
            console.log(r.code);
            if ("SUCCESS"===r.code) {
                location.href = '/';
            } else {
                $.modal.closeLoading();
                $('.imgcode').click();
                $(".code").val("");
                $.modal.msg(r.msg);
            }
        }
    });
}

function validateRule() {
    var icon = "<i class='fa fa-times-circle'></i> ";
    $("#signupForm").validate({
        rules: {
            username: {
                required: true
            },
            password: {
                required: true
            }
        },
        messages: {
            username: {
                required: icon + "请输入您的用户名",
            },
            password: {
                required: icon + "请输入您的密码",
            }
        }
    })
}