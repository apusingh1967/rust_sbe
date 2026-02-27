curl -O https://repo1.maven.org/maven2/uk/co/real-logic/sbe-all/1.37.0/sbe-all-1.37.0.jar

java -Dsbe.target.language=rust -Dsbe.output.dir=. -jar sbe-all-*.jar ./messages.xml 