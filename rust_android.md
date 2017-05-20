Ubuntu中Rust-Android编译环境搭建<br/>

参考连接:<br/>
https://github.com/rust-lang/rust-wiki-backup/blob/master/Doc-building-for-android.md<br/>
https://github.com/kennytm/rust-ios-android<br/>

1.下载NDK<br/>
	https://dl.google.com/android/repository/android-ndk-r14b-linux-x86_64.zip<br/>

2.解压<br/>
	/home/annrobot/rust/android-ndk-r14b<br/>

3.创建工具连<br/>
	cd ~/rust/android-ndk-r14b/build/tools<br/>
	./make-standalone-toolchain.sh --platform=android-14 --arch=arm --install-dir=/home/annrobot/ndk-standalone-14-arm<br/>

4.安装必要工具<br/>
	sudo apt-get install libc6-i386 lib32z1 lib32stdc++6<br/>

5.配置环境变量<br/>
	export ANDROID_NDK="$HOME/android-ndk-r14b"<br/>
	export ANDROID_TOOLCHAIN="/home/annrobot/ndk-standalone-14-arm"<br/>
	export PATH="$PATH:$ANDROID_TOOLCHAIN/bin"<br/>
	source ~/.bashrc<br/>
	
6.安装rust<br/>
运行 curl https://sh.rustup.rs -sSf | sh<br/>
按照提示进行安装<br/>

7.安装编译目标<br/>
#rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android<br/>
rustup target add arm-linux-androideabi<br/>

7.rustc编译<br/>
test.rs:<br/>
fn main(){
    println!("Hello World!");
}
<br/>
rustc --target=arm-linux-androideabi -C linker=$ANDROID_TOOLCHAIN/bin/arm-linux-androideabi-gcc -C link-args=-pie test.rs

8.cargo编译<br/>
配置:<br/>
vim /home/annrobot/.cargo/config :<br/>
[target.arm-linux-androideabi]
ar = "/home/annrobot/ndk-standalone-14-arm/bin/arm-linux-androideabi-ar"
linker = "/home/annrobot/ndk-standalone-14-arm/bin/arm-linux-androideabi-gcc"

编译:<br/>
cargo build --target arm-linux-androideabi --release<br/>

9.android项目<br/>
参考:https://docs.rs/jni/0.4.0/jni/<br/>

cargo.toml:<br/>
[package]<br/>
name = "lib_xor_net"<br/>
version = "0.1.0"<br/>
authors = ["JiaYe"]<br/>

[dependencies]<br/>

[target.'cfg(target_os="android")'.dependencies]<br/>
jni = { version = "0.4", default-features = false }<br/>

[lib]<br/>
name = "xor_net"<br/>
crate-type = ["staticlib", "cdylib"]<br/>


main.rs<br/>

#[no_mangle]<br/>
pub extern "C" fn receive(from: String) -> String {<br/>
    String::from(format!("{:?}:{:?}", from, "我是LibXorNet!"))<br/>
}<br/>

//定义Android JNI接口<br/>
#[cfg(target_os="android")]<br/>
#[allow(non_snake_case)]<br/>
pub mod android {<br/>
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_planet_rust_LibXorNet_receive(env: JNIEnv, _: JClass, from: JString) -> jstring {
        let name:String = env.get_string(from).expect("无法取到名字!").into();
        let echo = receive(name);
        let output = env.new_string(echo).expect("java字符串创建失败!");
        output.into_inner()
    }
}
<br/>
编译:<br/>
cargo build --target arm-linux-androideabi --release<br/>
将 ./target/arm-linux-androideabi/release/libxor_net.so 复制到android项目 /libs/armeabi/libxor_net.so<br/>

Java代码:<br/>

com.planet.rust.LibXorNet:<br/>
package com.planet.rust;<br/>

public class LibXorNet {<br/>
	static{<br/>
		System.loadLibrary("xor_net");<br/>
	}<br/>
	public static native String receive(String name);<br/>
}<br/>

<br/>
public class MainActivity extends Activity {

	@Override
	protected void onCreate(Bundle savedInstanceState) {
		super.onCreate(savedInstanceState);
		setContentView(R.layout.activity_main);
		
		String result = LibXorNet.receive("安卓");
		Toast.makeText(this, result, Toast.LENGTH_LONG).show();
	}
}
