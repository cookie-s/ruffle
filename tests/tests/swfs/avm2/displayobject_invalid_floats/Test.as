package {
    import flash.display.MovieClip;
    import flash.geom.Matrix;

    public class Test extends MovieClip {
        private var props:Array = ["rotation", "x", "y", "scaleX", "scaleY"];

        public function Test():void {
            TestZeroSkew();

            TestWeirdMatrix();
        }

        public function TestZeroSkew(): void {
            // The matrices are created by scaling and rotating.
            // They shouldn't have any private internal value like skew.

            var scaleVals1:Array = [0, NaN];
            var scaleVals2:Array = [1, NaN];
            var scaleVals3:Array = [1, 0];
            var rotations:Array = [0, 0.5, -0.5, 1, -1, 2, -2, 0.25, -0.25];
            for each (var prop:String in props) {
                trace("// TestZeroSkew: " + prop + " = 0");
                for(var idx1:int = 0; idx1 < 2; idx1++) {
                    for(var idx2:int = 0; idx2 < 2; idx2++) {
                        for(var idxr:int = 0; idxr < rotations.length; idxr++) {
                            var m:Matrix = new Matrix();

                            var clip:MovieClip = new MovieClip();
                            m.identity();
                            m.scale(scaleVals1[idx1], scaleVals1[idx2]);
                            m.rotate(rotations[idxr] * Math.PI);
                            clip.transform.matrix = m;
                            trace(m);
                            printChange(clip, prop, 0);
                            trace("");

                            m.identity();
                            m.scale(scaleVals2[idx1], scaleVals2[idx2]);
                            m.rotate(rotations[idxr] * Math.PI);
                            clip.transform.matrix = m;
                            trace(m);
                            printChange(clip, prop, 0);
                            trace("");

                            m.identity();
                            m.scale(scaleVals3[idx1], scaleVals3[idx2]);
                            m.rotate(rotations[idxr] * Math.PI);
                            clip.transform.matrix = m;
                            trace(m);
                            printChange(clip, prop, 0);
                            trace("");
                        }
                    }
                }
            }
        }

        public function TestWeirdMatrix(): void {
            // Assign arbitrary matrix to clip.transform.matrix and verify the change in property values.

            var matVals1:Array = [0, NaN];
            var matVals2:Array = [1, NaN];
            var matVals3:Array = [1, 0];
            var matVals4:Array = [-1, 0];
            var matVals5:Array = [2, 3];
            for each (var prop1:String in props) {
                trace("// " + prop1 + " = 0");
                for(var idx1:int = 0; idx1 < 2; idx1++) {
                    for(var idx2:int = 0; idx2 < 2; idx2++) {
                        for(var idx3:int = 0; idx3 < 2; idx3++) {
                            for(var idx4:int = 0; idx4 < 2; idx4++) {
                                var clip:MovieClip = new MovieClip();
                                clip.transform.matrix = new Matrix(matVals1[idx1], matVals1[idx2], matVals1[idx3], matVals1[idx4], 0, 0);
                                printChange(clip, prop1, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(matVals2[idx1], matVals2[idx2], matVals2[idx3], matVals2[idx4], 0, 0);
                                printChange(clip, prop1, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(matVals3[idx1], matVals3[idx2], matVals3[idx3], matVals3[idx4], 0, 0);
                                printChange(clip, prop1, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(matVals4[idx1], matVals4[idx2], matVals4[idx3], matVals4[idx4], 0, 0);
                                printChange(clip, prop1, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(matVals5[idx1], matVals5[idx2], matVals5[idx3], matVals5[idx4], 0, 0);
                                printChange(clip, prop1, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(matVals5[idx1], matVals5[idx2], matVals5[idx3], matVals5[idx4], 0, 0);
                                printChange(clip, prop1, 5);
                                trace("");
                            }
                        }
                    }
                }
            }

            return; // FIXME: Infinity/-Infinity should be tested. Remove this `return`;

            var matVals6:Array = [0, NaN, Infinity, -Infinity];
            for each (var prop2:String in props) {
                trace("// " + prop2 + " = NaN");
                for each (var l1 in matVals6) {
                    for each (var l2 in matVals6) {
                        for each (var l3 in matVals6) {
                            for each (var l4 in matVals6) {
                                var clip:MovieClip = new MovieClip();
                                clip.transform.matrix = new Matrix(l1, l2, l3, l4, 0, 0);
                                printChange(clip, prop2, 0);
                                trace("");
                            }
                        }
                    }
                }
            }
        }

        private function printChange(clip:MovieClip, prop:String, value:*): void {
            trace("  transform.matrix = " + clip.transform.matrix);
            var result:String = "";
            for each (var p1:String in props) {
                result += p1 + "=" + clip[p1] + ", ";
            }
            trace("  " + result);

            trace("clip[" + prop + "] = " + value);
            clip[prop] = value;

            result = "";
            for each (var p2:String in props) {
                result += p2 + "=" + clip[p2] + ", ";
            }
            trace("  " + result);
            trace("  transform.matrix = " + clip.transform.matrix);
        }
    }
}
