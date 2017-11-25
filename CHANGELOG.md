### 0.5.3

* Updated index find method to return `Path` references.
* Made `IndexedPath` type and its fields private.

### 0.5.2

* Updated fragment dependency to v0.3.1.
* Properly handle case insensitive indexing
    The previous implementation incorrectly modified the path itself when
    building case insensitive index entries. This implementation limits case
    sensitivity to a discrete search field, leaving the original file path
    untouched.

### 0.5.1

* Updated fragment dependency to v0.3.0.

### 0.5.0

* Case sensitivity is now set when populating the index. Converting string
  representations to lowercase is expensive; doing it once during index
  population (rather than for every subsequent call to find) is considerably
  more efficient.

### 0.4.0

* Added the ability to exclude entries using glob patterns when populating index.
* Updated walkdir dependency to v2.0.1.

### 0.3.0

* Add case_insensitive search option

### 0.2.4

* Update fragment dependency to v0.1.2.

### 0.2.3

* Use extracted fragment library for matching.

### 0.2.2

* Use walkdir crate instead of deprecated fs::walk_dir.

### 0.2.1

* Removed deprecated PathExt usage.

### 0.2.0

* Replace similarity algorithm with a space-delimited, term-driven implementation.
