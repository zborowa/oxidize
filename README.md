[![Build Status](https://travis-ci.org/zborowa/oxidize.svg?branch=master)](https://travis-ci.org/zborowa/oxidize) <a href="http://www.rascal-mpl.org"><img src="http://www.rascal-mpl.org/assets/img/logo_rascal_light_small.png" height="22px"></a>
<!-- ![Rascal Metaprogramming Language](http://www.rascal-mpl.org/assets/img/logo_rascal_light_small.png){:height="36px" width="36px"} -->
# Oxidize: Open framework for idiomatic rule preservation

[//]: # (ToDo: Write the project description)

This research investigates idioms in Rust programming language to improve both readability and code quality, by refactoring non-idiomatic code. This is done by enabling Rascal Metaprogramming Language to parse, analyse, and process the grammar of the Rust programming language.

# Requirements

Oxidize project can be run through a terminal or an Eclipse IDE. This is due to it being developed in Rascal MPL, which in turn works on the Java JVM.

## Eclipse
For the **Eclipse** usage of this project it is important to have the following program versions installed on your computer:
* [Eclipse for RCP Developers](http://www.eclipse.org/downloads/packages/release/Neon/3) >= Neon.2
  * [Rascal MPL](http://www.rascal-mpl.org/) == 0.8.4.201706151132
    * JDK >= 1.8
    * JRE >= 1.8

(Other versions have not been fully tested by this project)

## CLI
For the **CLI** usage of this project it is important to have the following program versions installed on your computer:
* [Rascal MPL](http://www.rascal-mpl.org/) == 0.8.4.201706151132
  * JDK >= 1.8
  * JRE >= 1.8

(Other versions have not been fully tested by this project)

# Usage

## Eclipse
Follow the installation steps for the Eclipse plugin on the Rascal MPL website: [Rascal MPL Start](http://www.rascal-mpl.org/start/)

(Steps on the website should reflect any changes to the environment of the Rascal MPL)

### To import
1. Start Eclipse
2. Click on *`File`*
   * Click on *`Open Projects from File System...`*
   * Import the project through the *`Directory...`*
   * Complete the steps through the wizard

### To Run
1. Open the *`Oxidize.rsc`* through the `Rascal Navigator`
2. Right click in the editor and click on the *`Start Console`* button
3. Now that we have a new console in the `Terminal` tab we can import the `Demo` module
   * This can be done by typing in the console the following: *`import Demo;`* (this module extends all the required modules to run the project)
4. This gives use the ability of running the project like follow: *`Oxidize(|<location>|, [options]);`*
   * Replace the `<location>` with the corresponding location of your to idiomatize project. Don't forget to have the vertical bars (`|`) around your location. That is a location type within Rascal MPL, just like how a text enclosed by double quotes (`"`) is a string.
   * We can also pass options to the function by adding them just like normal parameters of a function (at this time only one option exists):
     * `verbose=true`: returns additional information in the terminal for the user to better understand what is happening
5. Hitting *`Enter`* on your keyboard will run the function and should not take long to complete

Following these steps will create a new folder next to the given location with the same name followed by `_idiom`.

## CLI
1. Download the Oxidize project
2. Download the runnable .jar file from the Rascal MPL website: [Direct download link](https://update.rascal-mpl.org/console/rascal-shell-unstable.jar)
   * This project makes use of the .jar file created by the unstable branch of Rascal MPL (this may change in the future)
3. Put the Rascal MPL .jar file into the root of the Oxidize project
   * Where the *Oxidize.rsc* file resides
4. Run the project like follow: *`java -Xmx1G -Xss32m -jar <rascal-version>.jar Oxidize.rsc [-v] <location>`*
   * Replace the `<rascal-version>` with the corresponding name of the .jar file
   * Adding *`-v`* after the `Oxidize.rsc` will return additional information in the terminal for the user to better understand what is happening
   * Replace the `<location>` with the location of your to idiomatize project
5. Hitting *`Enter`* on your keyboard will run the function and should not take long to complete

Following these steps will create a new folder next to the given location with the same name followed by `_idiom`.
